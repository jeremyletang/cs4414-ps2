/*!
* gash : buitlins
* cs4414 ps2
* Jeremy Letang / free student
*/

#[allow(dead_code)];

use std::os;
use std::path::Path;

use error_code::{ErrorCode, Continue, Exit};

static builtins_list: [&'static str, ..4] = [
    &"exit",
    &"history",
    &"cd",
    &"alias"
];

pub struct Builtins {
    priv history: ~[~str],
    priv prev_dir: Path
}

impl Builtins {
    pub fn new() -> Builtins {
        Builtins {
            history: ~[],
            prev_dir: os::getcwd()
        }
    }

    pub fn execute(&mut self, cmd: &[~str]) -> (~str, ErrorCode) {
        match cmd[0] {
            ~"cd"       => self.execute_cd(cmd),
            ~"history"  => self.execute_history(),
            ~"exit"     => self.execute_exit(),
            ~"alias"     => self.execute_alias(),
            _           => unreachable!()
        }
    }

    pub fn save_cmd(&mut self, cmd: &str) {
        let mut c = cmd.to_owned();
        c.pop_char();
        self.history.push(c);
    }

    fn execute_cd(&mut self, cmd: &[~str]) -> (~str, ErrorCode) {
        if cmd.len() > 1 {
            let path = Path::new(cmd[1].clone());
            if path.is_dir() {
                self.prev_dir = os::getcwd();
                os::change_dir(&path);
                (~"", Continue)
            } else if path.is_file() {
                (format!("-gash: {}: Not a directory\n", cmd[1]), Continue)
            } else if cmd[1] == ~"-" {
                let path = self.prev_dir.clone();
                self.prev_dir = os::getcwd();
                os::change_dir(&path);
                (~"", Continue)
            } else { 
                (format!("-gash: {}: Not such file or directory\n", cmd[1]), Continue)
            }
        } else {
            (~"Not implemented\n", Continue)
        }
    }

    fn execute_history(&mut self) -> (~str, ErrorCode) {
        let mut hist: ~str = ~"";
        let mut index: i32 = 1;

        for s in self.history.iter() {
            hist.push_str(index.to_str());
            hist.push_str(&": ");
            hist.push_str(*s);
            hist.push_str(&"\n");
            index += 1;
        }

        (hist, Continue)
    }

    fn execute_exit(&mut self) -> (~str, ErrorCode) {
        (~"GoodBye !\n", Exit)
    }

    fn execute_alias(&mut self) -> (~str, ErrorCode) {
        (~"Not implemented\n", Continue)
    }
}

pub fn is_builtin(cmd: &str) -> bool {
    for s in builtins_list.iter() {
        if s.eq(&cmd) {
            return true;
        }
    }
    return false;
}
