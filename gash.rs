/*!
* gash : main crate
* cs4414 ps2
* Jeremy Letang / free student
*/

#[allow(unused_variable, unused_must_use)];

use std::{io, run, task, str, os};
use std::io::{File, Truncate, Write};
use std::io::buffered::BufferedReader;
use std::run::{Process, ProcessOptions};
use std::libc::c_int;
use std::libc;

use builtins::Builtins;
use error_code::{ErrorCode, Continue, Exit};

mod error_code;
mod builtins;
mod lex;
mod signal;

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

        loop {
            print!("{}", self.prompt);
            io::stdio::flush();

            let line: ~str = stdin.read_line().expect(format!("Error on file: {} at line {}", file!(), line!()));
            match lex::execute(line) {
                Ok(cmd)     => {
                    if cmd.len() != 0 {
                        if builtins::is_builtin(cmd[0]) {
                            match  self.builtins.execute(cmd) {
                                (out, Exit)     => { print!("{}", out); os::set_exit_status(0); break },
                                (out, _)        => { print!("{}", out) }
                            };
                        } else {
                            self.execute(cmd, 0, 1);
                            self.builtins.save_cmd(line);
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

    pub fn execute(&mut self, cmd: ~[~str], p_in: c_int, p_out: c_int) {
        if cmd.iter().position(|x| *x == ~"|").is_some() {
            self.pipe(cmd, p_in, p_out)
        } else if cmd.iter().position(|x| *x == ~"<").is_some() {
            self.redirect_left(cmd, p_in, p_out)
        } else if cmd.iter().position(|x| *x == ~">").is_some() {
            self.redirect_right(cmd, p_in, p_out)
        } else {
            self.simple_cmd(cmd, p_in, p_out)
        }

    }

    pub fn pipe(&mut self, cmd: ~[~str], p_in: c_int, p_out: c_int) {
        // (~"Not implemented\n", Continue)
    }

    pub fn redirect_left(&mut self, cmd: ~[~str], p_in: c_int, p_out: c_int) {
         // get the command
        let mut left_it = cmd.iter().take_while(|x| **x != ~"<");
        let mut right_it = cmd.iter().skip_while(|x| **x != ~"<");
        let mut left_cmd: ~[~str] = ~[];
        right_it.next();
        let file = right_it.next().unwrap().clone();
        // get the file
        let file_path  = Path::new(file.clone()); 
        if file_path.is_dir() {
           println!("-bash: {}: Is a directory", file);
           return;;
        }
        for c in left_it { left_cmd.push(c.clone()); }
        self.execute(left_cmd, fd_from_path(file_path, "r"), p_out);
    }

    pub fn redirect_right(&mut self, cmd: ~[~str], p_in: c_int, p_out: c_int) {
        // get the command
        let mut left_it = cmd.iter().take_while(|x| **x != ~">");
        let mut right_it = cmd.iter().skip_while(|x| **x != ~">");
        let mut left_cmd: ~[~str] = ~[];
        right_it.next();
        let file = right_it.next().unwrap().clone();
        // get the file
        let file_path  = Path::new(file.clone()); 
        if file_path.is_dir() {
           println!("-bash: {}: Is a directory", file);
           return;;
        }
        for c in left_it { left_cmd.push(c.clone()); }
        self.execute(left_cmd, p_in, fd_from_path(file_path, "w"));
    }

    pub fn simple_cmd(&mut self, mut cmd: ~[~str], p_in: c_int, p_out: c_int) {
            if !cmd_exist(cmd[0]) {
               println!("-gash: {}: command not found", cmd[0]);
               return;
            }
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
               print!("")
            } else {
                let prg = cmd.shift_opt().expect(format!("Error on file: {} at line {}", file!(), line!()));
                let mut proc_res = Process::new(prg, cmd, 
                    ProcessOptions { env: None, dir: None, in_fd: Some(p_in), out_fd: Some(p_out), err_fd: Some(2)}).unwrap();
                if p_in != 0 {os::close(p_in);}
                if p_out != 1 {os::close(p_out);}
                proc_res.finish();
                // io::stdio::flush();
            }
    }

}

fn cmd_exist(cmd: &str) -> bool {
    run::process_output("which", [cmd.to_owned()]).unwrap().status.success()
}

fn fd_from_path(file_path: Path, args: &str) -> i32 {
    unsafe {
       args.with_c_str(|c_args| {
            libc::fileno(libc::fopen(file_path.to_c_str().unwrap(), c_args))
        })
    }
}

pub fn main() {

    let mut sh = Sh::new();
    sh.run();
}