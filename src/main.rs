// ================================================================== 
// neovim daily training - 12-15 minute workout
// ==================================================================
// custom bindings: j=up, k=down
// note: always start by setting a mark before each exercise
// move to the first line of code(not function) then follow the
// instruction below.
// after every change you make if in insert mode press esc u to undo
// ==================================================================

// ma set a mark here by pressing ma 'a to jump back here
fn exercise_a() { // basic navigation - learn to move around 
    // k=down 4x to next line
    println!("you made it here!");
    // j=up 1x to go back
    // k=down 3x to next line
    println!("now go back up!");
    // $=end of line, 0=start of line, l=right, h=left repeatedly
    println!("move through this line using l and h!");
    // w=next word, b=back word, e=end of word
    println!("if it works but you don't understand why,
it's only a matter of time before it breaks.");
    // f and F jump to char: fi=jump forward to 'i', Fi=back to 'i'
    // t and T jump before char: tw=before 'w', Tw=back before 'w'
    // ;=repeat last f/F/t/T, ,=repeat in opposite direction
    // 'a returns to mark
}

// mb
fn exercise_b() { // jumps
    let first = 1;
    // ''=jump to previous position 'b returns
    // press x to delete a char, k=down, `.=jump to last edit
    // gg=top of file, 'b=back here, G=bottom of file, 'b=back
    // 10G=jump to line 10, :50 enter=jump to line 50, 'b
    // ctrl+d=half page down, ctrl+u=half page up
    // ctrl+f=full page down, ctrl+b=full page up
    // H=top of screen, M=middle of screen, L=bottom of screen
    // {=up one paragraph, }=down one paragraph
    // 'b returns to mark
}

// mc
fn exercise_c() { // file handling - shell commands, file ops, execution
    // SHELL COMMANDS FROM VIM:
    // :!ls enter=list files in current directory
    // :!ls -la enter=detailed file listing
    // :pwd enter=print working directory
    // :cd .. enter=change directory up one level
    // :cd ~/projects enter=change to specific directory
    // :!mkdir test_folder enter=create new folder
    // :!rm test.txt enter=delete file (careful!)
    // :!mv old.txt new.txt enter=rename/move file
    // :!cp file.txt backup.txt enter=copy file
    
    // EXECUTE CODE FROM VIM:
    // :!python test.py enter=run python script
    // :!python3 % enter=run current file (% = current filename)
    // :!cargo run enter=run rust project
    // :!node script.js enter=run javascript
    // :!gcc % -o output && ./output enter=compile and run c
    // :!python -m http.server 8000 enter=start local server
    // ctrl+z=suspend vim, fg enter=return to vim (for long processes)
    
    // FILE NAVIGATION:
    // :Ex enter=explore files (netrw)
    // :Sex enter=split window explore
    // :Vex enter=vertical split explore
    // i in netrw=cycle view modes, enter=open file, -=go up directory
    
    // READ/WRITE FILE OPERATIONS:
    // :r filename.txt enter=read file into current buffer below cursor
    // :r !ls enter=read shell command output into buffer
    // :w newname.txt enter=save current buffer as new file
    // :saveas newfile.rs enter=save as and switch to new file
    // :'<,'>w partial.txt enter=write visual selection to new file
    
    // DIFF AND COMPARE:
    // :!diff file1.txt file2.txt enter=compare two files
    // :vert diffsplit other.rs enter=open diff in split
    
    // WORKFLOW PRACTICE:
    // 1. :pwd to check current directory
    // 2. :!ls to see available files
    // 3. :!mkdir practice_folder to create test folder
    // 4. :e practice_folder/test.py to create new python file
    // 5. type: print("hello from vim") then :w to save
    // 6. :!python % to execute the python file
    // 7. :!ls practice_folder to verify file exists
    // 8. :!python -m http.server 8000 & to start server in background
    // 9. create index.html: :e practice_folder/index.html
    // 10. add: <h1>Testing</h1> then :w
    // 11. :!curl localhost:8000/index.html to test (or open browser)
    // 12. ctrl+z to suspend vim and check browser
    // 13. fg to return to vim
    // 14. :r !date to insert current date/time into file
    // 15. :!ps aux | grep python to check running processes
    // 16. :!kill %1 or pkill -f "http.server" to stop server
    // 17. :!cp practice_folder/test.py practice_folder/backup.py
    // 18. :!diff practice_folder/test.py practice_folder/backup.py
    // 19. :cd .. to go back up
    // 20. :!rm -rf practice_folder to clean up (careful!)
    
    // 'c returns to mark
}

// md
fn exercise_d() { // buffer and split management - multiple files at once
    // BUFFER BASICS:
    // :ls enter=list all open buffers
    // :ls! enter=list all buffers including unlisted
    // :b2 enter=switch to buffer 2
    // :b init enter=switch to buffer by name (tab complete works)
    // :bn enter=next buffer
    // :bp enter=previous buffer
    // <leader>j=previous buffer
    // <leader>k=next buffer
    // :bf enter=first buffer
    // :bl enter=last buffer
    
    // OPENING FILES TO BUFFERS:
    // :e newfile.txt enter=open/create file (adds to buffer list)
    // :e ../path/file.rs enter=open file with relative path
    // :e ~/projects/main.rs enter=open with absolute path
    // :enew enter=create unnamed buffer (:w filename.txt later to save)
    // :e ../tab shows all files in parent directory
    
    // HORIZONTAL SPLITS:
    // :split enter=split current buffer horizontally
    // :split other.rs enter=horizontal split opening other.rs
    // :sp enter=shorthand for split
    // :10split enter=split with 10 line height
    // :sb 2 enter=split and show buffer 2
    // ctrl+w s=split current window
    
    // VERTICAL SPLITS:
    // :vsplit enter=split current buffer vertically
    // :vsplit main.rs enter=vertical split opening main.rs
    // :vsp enter=shorthand for vsplit
    // :30vsplit enter=split with 30 char width
    // :vert sb 3 enter=vertical split showing buffer 3
    // ctrl+w v=split current window vertically
    
    // NAVIGATING SPLITS:
    // ctrl+w h=move to left split
    // ctrl+w l=move to right split
    // ctrl+w j=move to split above (remember j=up)
    // ctrl+w k=move to split below (remember k=down)
    // ctrl+w w=cycle through all splits
    // ctrl+w p=move to previous split
    
    // RESIZING SPLITS:
    // ctrl+w +=increase current split height
    // ctrl+w -=decrease current split height
    // ctrl+w >=increase current split width
    // ctrl+w <=decrease current split width
    // ctrl+w ==make all splits equal size
    // :resize 20 enter=set height to 20 lines
    // :vertical resize 50 enter=set width to 50 chars
    
    // CLOSING SPLITS AND BUFFERS:
    // :q enter=quit current split (buffer stays open)
    // :close enter=close current split
    // :only enter=close all splits except current
    // ctrl+w c=close current split
    // ctrl+w o=close all splits except current
    // :bd enter=delete current buffer (closes from buffer list)
    // :bd 3 enter=delete buffer 3
    // :bd init enter=delete buffer by name
    // :bd! enter=force delete even if unsaved
    // <leader>bd=delete current buffer
    
    // MOVING SPLITS:
    // ctrl+w H=move split to far left (full height)
    // ctrl+w L=move split to far right (full height)
    // ctrl+w J=move split to top (full width, remember j=up)
    // ctrl+w K=move split to bottom (full width, remember k=down)
    // ctrl+w r=rotate splits
    // ctrl+w x=exchange with next split
    
    // WORKFLOW PRACTICE:
    // 1. open vim with: nvim file1.rs file2.lua file3.txt
    // 2. :ls to see all three buffers loaded
    // 3. :vsplit to create vertical split of current file
    // 4. :b2 to switch right split to buffer 2
    // 5. ctrl+w h to move to left split
    // 6. :split to create horizontal split in left panel
    // 7. :b3 to show buffer 3 in new split
    // 8. ctrl+w w to cycle through all three visible splits
    // 9. ctrl+w >= several times to widen a split
    // 10. ctrl+w == to equalize all splits
    // 11. ctrl+w k to move to bottom split (k=down)
    // 12. :e test.md to create new file in this split
    // 13. :ls to see test.md added as buffer 4
    // 14. type some text, :w to save test.md
    // 15. :bd test.md to close test.md buffer
    // 16. :b1 to switch current split to buffer 1
    // 17. ctrl+w j to move to top split (j=up)
    // 18. :close to close this split
    // 19. ctrl+w K to move remaining left split to bottom (full width)
    // 20. :only to close all but focused split
    // 21. :bn :bn :bp to practice buffer navigation
    // 22. :vsplit | :b2 to split and immediately show buffer 2
    // 23. ctrl+w L to move it to far right
    // 24. :vert sb 3 to add vertical split with buffer 3
    // 25. ctrl+w x to exchange split positions
    
    // 'd returns to mark
}

// me
fn exercise_e() { // global marks - jump between files instantly
    // LOCAL VS GLOBAL MARKS:
    // ma-mz=local marks (lowercase, per file only)
    // mA-mZ=global marks (uppercase, work across all files)
    // 'a=jump to local mark a in current file
    // 'A=jump to global mark A (switches files if needed)
    
    // SETTING GLOBAL MARKS:
    // mA=set global mark A at cursor position
    // mB, mC, mD... up to mZ for different locations
    // marks persist even after closing vim (in .viminfo)
    
    // VIEWING ALL MARKS:
    // :marks enter=show all marks (local and global)
    // :marks ABC enter=show only marks A, B, and C
    // :delmarks A enter=delete mark A
    // :delmarks! enter=delete all lowercase marks in buffer
    
    // JUMPING WITH MARKS:
    // 'A=jump to line of mark A (different file if needed)
    // `A=jump to exact position (line and column) of mark A
    // g'A=jump to mark A without changing jumplist
    // g`A=jump to exact position without changing jumplist
    
    // SPECIAL MARKS (automatic):
    // ''=jump back to position before last jump
    // '.=jump to last edit position
    // '^=jump to last insert position
    // '[=jump to start of last change/yank
    // ']=jump to end of last change/yank
    // '<='>=start/end of last visual selection
    
    // WORKFLOW PRACTICE (multi-file):
    // 1. open 3 files: nvim config.rs main.rs utils.rs
    // 2. in config.rs, go to important line, mA to set global mark
    // 3. :bn to switch to main.rs
    // 4. find important function, mB to set global mark
    // 5. :bn to switch to utils.rs
    // 6. find key function, mC to set global mark
    // 7. 'A to instantly jump back to config.rs at mark A
    // 8. 'B to jump to main.rs at mark B
    // 9. 'C to jump to utils.rs at mark C
    // 10. 'B to jump back to main.rs
    // 11. 'A to jump to config.rs
    // 12. :marks to see all your global marks listed
    // 13. :e lib.rs to open fourth file
    // 14. add code, mD to mark this location
    // 15. 'A 'B 'C 'D to practice jumping between all 4 files
    // 16. '' to jump back to previous position
    // 17. '. to jump to last edit you made
    // 18. make an edit in lib.rs
    // 19. 'B to jump to main.rs
    // 20. '. to jump back to your edit in lib.rs
    // 21. :marks ABCD to view just your global marks
    // 22. :delmarks D to delete mark D
    // 23. 'C, make edit, '^ to return to that insert position
    // 24. visual select some text, 'A to jump away
    // 25. '< to jump back to start of that selection
    
    // PRO WORKFLOW (advanced):
    // 1. working on feature: set mF in feature file
    // 2. checking tests: set mT in test file
    // 3. reviewing docs: set mD in documentation
    // 4. quick config check: set mC in config
    // 5. now jump instantly: 'F 'T 'D 'C as needed
    // 6. no more :b or :bn cycling through files
    // 7. use lowercase marks (ma-mz) for within-file positions
    // 8. use uppercase marks (mA-mZ) for cross-file landmarks
    
    // 'e returns to mark
}

// mf
fn exercise_f() { // search and find - locate text quickly
    let new_delta = 42;
    // move onto "new_delta"
    // *=search forward, #=search back, n=next N=previous match
    let new_delta_squared = new_delta * new_delta;
    // search word /squared enter, ?squared enter forward-back n=next N=prev
    // :noh enter=turn off search highlighting
    // 'f returns to mark
}

// mg
fn exercise_g() { // insert mode and line joining
    println!
("hello world")
    // move to hello world, A=append at end, type ;, esc u=undo
    // j=up a line gJ=join lines, J join w/space
    println!("day one")
    // I=insert at start, type // , esc (comments it)
    // o=open line below, O=open line above, type something
    // 'g returns to mark
}

// mh
fn exercise_h() { // deletion - basic and advanced text objects
    let useless_variable = 123;
    // dd=delete line u=undo
    let another_useless = 456;
    // 0 then d$=delete to end
    let delete_me_please = 789;
    // move onto "delete_me_please", diw=delete inner word
    // 0 then w then dw=delete first word
    // $ then d0=delete end to start
    // x=delete single char
    let values = (100, 200, 300);
    // f(=jump to paren, di(=delete inside parens
    // di plus ( or [ or { or " deletes inside ([{"
    // da plus ( or [ or { or " deletes around ([{"
    let message = "delete the text inside these quotes";
    // f"=jump to quote, di"=delete inside ""
    let array = [1, 2, 3];
    // f[=jump to [, di[=delete inside []
    let code = { println!("inside braces"); };
    // f{=jump to brace, di{=delete inside {}
    let sentence = "delete until semicolon; keep this";
    // dt;=delete up to ;
    // df;=delete including ;
    // 'h returns to mark
}

// mi
fn exercise_i() { // change commands and case changes
    let old_name = "change me";
    // move to "old_name", ciw=change inner word. esc, u=undo
    println!("Replace this text");
    // 0 then fR, cw=change word, type debug
    // cc=change entire line, type new line
    let value = (100, 200);
    // f(=jump to (, ci(=change inside ()
    let text = "modify this string";
    // f"=jump to ", ci"=change inside ""
    // C=change to end of line, type something
    let UPPERCASE = "lowercase";
    // fU, guiw=lowercase word
    // fl, gUiw=uppercase word
    // move over any letter, ~=toggle case repeatedly
    // 'i returns to mark
}

// mj
fn exercise_j() { // visual mode - select char, line, and block
    let second_line = 2; // 0 then set mark mz here
    let third_line = 3;
    // 'z, V=visual line mode, k=extend down, d=delete
    let word1 = 1;

    // fw, v=visual mode, e=extend to end, y=yank, k p=paste
    let col1 = 10;
    let col2 = 20;
    let col3 = 30;
    // www to =, ctrl+q=visual block, k 2x, l 4x, d=delete
    // select all: ggVG gg=top, V=visual line, G=bottom
    // 'j returns to mark
}

// mk
        fn exercise_k() { // formatting
    // << or >> tab an individual line back or forward

let badly_indented = 1;
let another_bad = 2;
let way_too_far = 3;

    // vip=select inner paragraph, then = to auto-indent
    // vap=select around includes trailing space, d=delete

        let misaligned_block = {
            let x = 1;
        let y = 2;
                let z = 3;
        };

    // vip>=indent right
    // vip<=un-indent left
    // vipy=yank paragraph, k=down paragraph, p=paste
    // 'k returns to mark
}

// ml
fn exercise_l() { // copy and paste - yank w/ registers and clipboard
    let copy_this_line = 999;
    // yy=yank line, k=down, p=paste below, P=paste above
    let duplicate = "copy me";
    // move to "duplicate", yiw=yank word, $, p=paste
    let register_practice = "use named registers";
    // "ayy=yank to register a, k, "ap or "aP=paste below/above
    let word_one = 1;
    let word_two = 2;
    // move to "word_one", "byw=yank word to register b, k=down, diw "bp=paste
    let clipboard_test = "copy to system clipboard";
    // "+yy=yank to clipboard, "+p or "+P paste below/above
    // y$=yank to end, y0=yank to start
    // 'l returns to mark
}

// mm
fn exercise_m() { // edit operations - find/replace, increment/decrement
    let new_list = vec![1, 2, 3];
    let new_list_length = new_list.len();
    // :%s/new_list/old_list/g or /gc replace all or w/check
    // :s/new_list/old_list/ or /g replace first in line or all
    let counter = 0;
    // f0=jump to 0, ctrl+a=increment (0→1)
    // ctrl+x=decrement (1→0)
    let numbers = [10, 20, 30, 40];
    // f1=jump to 10, ctrl+a (10→11), f2 ctrl+a f3 ctrl+a u=undo all changes
    // 'm returns to mark
}

// mn
fn exercise_n() { // power moves - macros, dot command
    let item1 = 1;
    let item2 = 2;
    // move to item1, qq=record, ciw type element esc, q=stop record
    // move to item2, @q=replay macro, @@=replay last macro
    let value1 = 10;
    let value2 = 20;
    let value3 = 30;
    // move to value1, ciw type number esc, move to value2, .=repeat last change
    // 'n returns to mark
}

fn main() {
    
}

// ==================================================================
// quick reference cheat sheet
// ==================================================================
// marks: m<letter> '<letter> '' `.
// navigation: h,j(up),k(down),l  w,b,e  0,$  gg,G  {,}  H,M,L
//             f/F/t/T ;,  ctrl+f/b ctrl+d/u :n nG
// search: /text ?text n,N *,# :noh
// insert: i,a,I,A,o,O
// delete: x,dd,dw,d$,d0,diw,di(,di{,di",di[,dt<c>,df<c>
// change: cw,ciw,ci",ci(,ci{,cc,C
// case: ~ guiw gUiw
// visual: v,V,ctrl+q  vip,vap  ggVG
// copy/paste: yy,yw,yiw,y$,y0,p,P  "ay "ap "+y "+p
// numbers: ctrl+a ctrl+x
// undo/redo: u,ctrl+r,U
// find/replace: :%s/old/new/g  :%s/old/new/gc
// repeat: . @<letter> @@
// line join: J gJ
// indent: >ip <ip =ip
// buffers: :ls :bn :bp :b# :bd :e file
// splits: :split :vsplit ctrl+w hjkl ctrl+w c :only
// shell: :!cmd :pwd :cd :r !cmd
// ==================================================================
