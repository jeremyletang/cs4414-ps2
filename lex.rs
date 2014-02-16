/*!
* gash : lex
* cs4414 ps2
* Jeremy Letang / free student
*/

enum State {
    Regular,
    String
}

static special_chars: [char, ..4] = ['|', '&', '<', '>'];

fn syntax_error(cmds: &[~str], c2: char) -> Result<(), ~str> {
    match special_chars.iter().find(|x| **x == c2).is_some() {
        false   => Ok(()),
        true    => {
            if cmds.len() == 0 {
                Err(format!("-gash: syntax error near unexpected token \'{}\'\n", c2))
            } else {
                let c1 = cmds.last_opt().unwrap().char_at(0);
                if special_chars.iter().find(|x| **x == c1).is_some() {
                    Ok(())
                } else {
                    Err(format!("-gash: syntax error near unexpected token \'{}\'\n", c2))
                }
            }
        }
    }
}

fn last_token_is_valid(token: char) -> bool {
    if token == '|' || token == '<' || token == '>' {
        false
    } else {
        true
    }
}

pub fn execute(cmd: &str) -> Result<~[~str], ~str> {
    let mut cmds: ~[~str] = ~[];
    let mut cur_str: ~str = ~"";
    let mut state = Regular;

    for c in cmd.chars() {
        match state {
            String      => {
                if c == '\"' {
                    state = Regular;
                    cmds.push(cur_str);
                    cur_str = ~"";
                } else {
                    cur_str.push_char(c);
                }
            },
            Regular     => {
                match c {
                    '\n'    => { if cur_str != ~"" { cmds.push(cur_str); cur_str = ~""} },
                    ' '     => { if cur_str != ~"" { cmds.push(cur_str); cur_str = ~""; } },
                    '|'     => { if cur_str != ~"" { cmds.push(cur_str); cur_str = ~""; } cmds.push(~"|") },
                    '>'     => { if cur_str != ~"" { cmds.push(cur_str); cur_str = ~""; } cmds.push(~">") },
                    '<'     => { if cur_str != ~"" { cmds.push(cur_str); cur_str = ~""; } cmds.push(~"<") },
                    '&'     => { if cur_str != ~"" { cmds.push(cur_str); cur_str = ~""; } cmds.push(~"&") },
                    '\"'    => { if cur_str != ~"" { cmds.push(cur_str); cur_str = ~""; } state = String },
                    ch      => cur_str.push_char(ch),
                }
                match syntax_error(cmds, c) {
                    Ok(_)   => {/* Nothing to do */},
                    Err(s)  => return Err(s)
                }
            }
        }
    }
    if cur_str != ~"" { cmds.push(cur_str); }
    if cmds.len() == 0 { return Ok(cmds); }
    match last_token_is_valid(cmds.last_opt().unwrap().char_at(0)) {
        true    => Ok(cmds),
        false   => Err(~"-gash: syntax error near unexpected token \'newline\'\n")
    }
}
