/*!
* gash : main crate
* cs4414 ps2
* Jeremy Letang / free student
*/

#[allow(unused_variable, unused_must_use)];

use std::{io, run, task, str};
use std::io::{File, Truncate, Write};
use std::io::buffered::BufferedReader;
use std::run::{Process, ProcessOptions};
use std::path::Path;

use builtins::Builtins;
use error_code::{ErrorCode, Continue, Exit};

mod error_code;
mod builtins;
mod lex;

struct Sh {
    prompt:    ~str,
    builtins:  Builtins,
    chan:      SharedChan<~str>,
    port:      Port<~str>
}

impl Sh {
    pub fn new() -> Sh {
        let (port, chan) = SharedChan::new();
        Sh {
            prompt:     ~"?> ",
            builtins:   Builtins::new(),
            chan:       chan,
            port:       port
        }
    }

    pub fn run(&mut self) {
        let mut stdin = BufferedReader::new(io::stdin());
        let mut result: (~str, ErrorCode);

        loop {
            print!("{}", self.prompt);
            io::stdio::flush();

            let line: ~str = stdin.read_line().expect(format!("Error on file: {} at line {}", file!(), line!()));
            match lex::execute(line) {
                Ok(cmd)     => {
                    if cmd.len() != 0 {
                        result = self.execute(cmd);
                        self.builtins.save_cmd(line);
                        match result {
                            (out, Exit)     => { print!("{}", out); break },
                            (out, _)        => { print!("{}", out) }
                        }
                    }
                },
                Err(err)    => print!("{}", err)
            }

            match self.port.try_recv() {
                Some(t) => print!("{}", t),
                None    => {/* nothing to do */}
            }
        }
    }

    pub fn execute(&mut self, cmd: ~[~str]) -> (~str, ErrorCode) {
        if cmd.iter().position(|x| *x == ~"|").is_some() {
            self.pipe(cmd)
        } else if cmd.iter().position(|x| *x == ~"<").is_some() {
            self.redirect_left(cmd)
        } else if cmd.iter().position(|x| *x == ~">").is_some() {
            self.redirect_right(cmd)
        } else {
            self.simple_cmd(cmd)
        }

    }

    pub fn pipe(&mut self, cmd: ~[~str]) -> (~str, ErrorCode) {
        (~"Not implemented\n", Continue)
    }

    pub fn redirect_left(&mut self, cmd: ~[~str]) -> (~str, ErrorCode) {
        (~"Not implemented\n", Continue)
    }

    pub fn redirect_right(&mut self, cmd: ~[~str]) -> (~str, ErrorCode) {
        // get the command
        let mut left_it = cmd.iter().take_while(|x| **x != ~">");
        let mut right_it = cmd.iter().skip_while(|x| **x != ~">");
        let mut left_cmd: ~[~str] = ~[];
        right_it.next();
        let file = right_it.next().unwrap().clone();
        // get the file
        let file_path  = Path::new(file.clone()); 
        if file_path.is_dir() {
            return (format!("-bash: {}: Is a directory\n", file), Continue);
        }
        let mut fd = File::open_mode(&file_path, Truncate, Write);
        for c in left_it { left_cmd.push(c.clone()); }
        let (res, err) = self.execute(left_cmd);
        fd.write(res.as_bytes());
        (~"", Continue)
    }

    pub fn simple_cmd(&mut self, mut cmd: ~[~str]) -> (~str, ErrorCode) {
        if builtins::is_builtin(cmd[0]) {
            self.builtins.execute(cmd)
        } else {
            if *cmd.last_opt().expect(format!("Error on file: {} at line {}", file!(), line!())) == ~"&" {
                let prg = cmd.shift_opt().expect(format!("Error on file: {} at line {}", file!(), line!()));
                cmd.pop();
                let chan = self.chan.clone();
                task::spawn(proc() {
                    let result = match Process::new(prg, cmd, ProcessOptions::new()) {
                        Some(mut pr) => {
                            let out = pr.finish_with_output();
                            let mut r = format!("\n[proc {}] Done: {}\n", pr.get_id() as i32, prg);
                            unsafe {
                                r.push_str(str::raw::from_utf8(out.output).to_owned());
                                r.push_str(str::raw::from_utf8(out.error).to_owned());
                            }
                            r
                        },
                        None => format!("-gash: {}: command not found\n", prg)
                    };
                    
                    chan.send(result)
                });

                (format!(""), Continue)
            } else {
                let prg = cmd.shift_opt().expect(format!("Error on file: {} at line {}", file!(), line!()));
                match run::process_output(prg, cmd) {
                    Some(out)     => {
                        let mut result;
                        unsafe {
                            result = str::raw::from_utf8(out.output).to_owned();
                            result.push_str(str::raw::from_utf8(out.error).to_owned());
                        }
                        (result, Continue)
                    },
                    None      => (format!("-gash: {}: command not found\n", prg), Continue)
                }
            }
        }
    }

}


pub fn main() {

    let mut sh = Sh::new();
    sh.run();
}