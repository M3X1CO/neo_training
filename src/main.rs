// ================================================================== 
// NEOVIM DAILY TRAINING - 12-15 MINUTE WORKOUT
// ==================================================================
// Custom bindings: j=UP, k=DOWN
// NOTE:Always start by setting a mark before each exercise
// Move to the first line of code(not function) then follow the
// instruction below.
// After every change you make if in insert mode press ESC u to undo
// ==================================================================

// ma set a mark here by pressing ma 'a to jump back here
fn exercise_a() { // BASIC NAVIGATION - Learn to move around 
    // k=down 4x to next line
    println!("You made it here!");
    // j=up 1x to go back
    // k=down 3x to next line
    println!("Now go back up!");
    // $=end of line, 0=start of line, l=right, h=left repeatedly
    println!("Move through this line using l and h!");
    // w=next word, b=back word, e=end of word
    println!("If it works but you don't understand why,
it's only a matter of time before it breaks.");
    // f and F jump to char: fI=jump forward to 'I', FI= back to 'I'
    // t and T jump before char: tw=before 'w', Tw=back before 'w'
    // ;=repeat last f/F/t/T, ,=repeat in opposite direction
    // 'a returns to mark
}

// mb
fn exercise_b() { // MARKS AND JUMPS - persist thru file closing
    let first = 1; // 0 to start, mx to set mark x, k move down
    let second = 2; // 0 to start, my to set mark y
    // 'b=jump to mark b, 'x=jump to mark x, 'y=jump to mark y
    // ''=jump to previous position
    // Press x to delete a char, k=down, `.=jump to last made edit
    // mA-Z sets global marks. You can hop between two files
    // File mA and mB then 'A will take you to file A, 'B takes you 
    // to file B fast tab through files your working
    // gg=top of file, 'b=back here, G=bottom of file, 'b=back
    // 10G=jump to line 10, :50 ENTER=jump to line 50, 'b
    // Ctrl+d=half page down, Ctrl+u=half page up
    // Ctrl+f=full page down, Ctrl+b=full page up
    // H=top of screen, M=middle of screen, L=bottom of screen
    // {=up one paragraph ¶, }=down one paragraph ¶
    // 'b returns to mark
}

// mc
fn exercise_c() { // SEARCH AND FIND - Locate text quickly
    let new_delta = 42;
    // Move onto "new_delta"
    // *=search forward, #=search back, n=next N=previous match
    let new_delta_squared = new_delta * new_delta;
    // search word /squared, ?squared ENTER forward-back n=next N=prev
    // :noh ENTER=  nooo.... no search highlighting
    // 'c returns to mark
}

// md
fn exercise_d() { // INSERT MODE and LINE JOINING
    println!
("Hello World");
    // Move to Hello World, A=append at end, type ;, ESC u=undo
    // j=up a line gJ=join lines, J join w/space
    println!("day one")
    // I=insert at start, type // , ESC (comments it)
    // o=open, O=open line below-above, type something
    // 'd returns to mark
}

// me
fn exercise_e() { // DELETION - Basic and advanced text objects
    let useless_variable = 123;
    // dd=delete line u=undo
    let another_useless = 456;
    // 0 then d$=delete to end
    let delete_me_please = 789;
    // Move onto "delete_me_please", diw=delete inner word
    // 0 then w then dw=delete first word
    // $ then d0=delete end to start
    // x=delete single char
    let values = (100, 200, 300);
    // f(=jump to paren, di(=delete inside parens
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
    let with_spaces = "word here";
    // w until you land on word, diw=delete inner word
    // 'e returns to mark
}

// mf
fn exercise_f() { // CHANGE COMMANDS and CASE CHANGES
    let old_name = "change me";
    // Move to "old_name", ciw=change inner word. ESC, u=undo
    println!("Replace this text");
    // 0 then fR, cw=change word, type debug
    // cc=change entire line, type new line
    let value = 100 + 200;
    // f(=jump to (, ci(=change inside ()
    let text = "modify this string";
    // f"=jump to ", ci"=change inside ""
    // C=change to end of line, type something
    let UPPERCASE = "lowercase";
    // fU, guiw=lowercase word
    // flfl, gUiw=uppercase word
    // Move over any letter, ~=toggle case repeatedly
    // 'f returns to mark
}

// mg
fn exercise_g() { // VISUAL MODE - Select char, line, and block
    let second_line = 2; // 0 then set mark mz here
    let third_line = 3;
    // 'z, V=visual line mode, k=extend down, d=delete
    let word1 = 1;

    // fw, v=visual mode, e=extend to end, y=yank, k P=paste
    let col1 = 10;
    let col2 = 20;
    let col3 = 30;
    // www to =, Ctrl+q=visual block, k 2x, l 4x, d=delete
    // SELECT ALL: ggVG gg=top, V=visual line, G=bottom
    // 'g returns to mark
}

// mh
        fn exercise_h() { // FORMATTING
    // << or >> tab an individual line back or forward

let badly_indented = 1;
let another_bad = 2;
let way_too_far = 3;

    // vip=select inner ¶, then = to auto-indent
    // vap=select around includes trailing space ¶, d=delete

        let misaligned_block = {
            let x = 1;
        let y = 2;
                let z = 3;
        };

    // vip>=indent right
    // vip<=un-indent left
    // vipy=yank paragraph, }=down ¶, p=paste
    // 'h returns to mark
}

// mi
fn exercise_i() { // COPY AND PASTE - Yank w/ registers ® and clipboard
    let copy_this_line = 999;
    // yy=yank line, k=down, p=paste below, P=paste above
    let duplicate = "copy me";
    // Move to "duplicate", yiw=yank word, $, p=paste
    let register_practice = "use named registers";
    // "ayy=yank to ® a, k, "ap or "aP=paste below/above
    let word_one = 1;
    let word_two = 2;
    // M2 "word_one", "byw=yank word ® b, k=down, diw "bP=paste
    let clipboard_test = "copy to system clipboard";
    // "+yy=yank to clipboard, "+p /P paste below/above
    // y$=yank to end, y0=yank to start
    // 'i returns to mark
}

// mj
fn exercise_j() { // EDIT OPERATIONS - Find/replace, increment/decrement
    let new_list = vec![1, 2, 3];
    let new_list_length = new_list.len();
    // :%s/new_list/old_list/g /gc replace all or w/check
    // :s/new_list/old_list/ /g replace first in line or all
    let counter = 0;
    // Move to line, f0=jump to 0, Ctrl+a=increment (0→1)
    // Ctrl+x=decrement (2→1)
    let numbers = [10, 20, 30, 40];
    // f1=jump to 10, Ctrl+a (10→11), f2 ^a f3 ^a U=undo all changes
    // 'j returns to mark
}

// mk
fn exercise_k() { // POWER MOVES - Macros, dot command
    let item1 = 1;
    let item2 = 2;
    // M2 item1, qq=record, ciw type element ESC, q=stop record
    // M2 item2, @q=replay macro, @@=replay last macro
    let value1 = 10;
    let value2 = 20;
    let value3 = 30;
    // M2 value1, ciw type number ESC, M2 value2, .=repeat last change
    // 'k returns to mark
}

fn main() {
    
}

// ==================================================================
// QUICK REFERENCE CHEAT SHEET
// ==================================================================
// MARKS: m<letter> '<letter> '' `.
// NAVIGATION: h,j(UP),k(DOWN),l  w,b,e  0,$  gg,G  {,}  H,M,L
//             f/F/t/T ;,  Ctrl+f/b Ctrl+d/u :N NG
// SEARCH: /text ?text n,N *,# :noh
// INSERT: i,a,I,A,o,O
// DELETE: x,dd,dw,d$,d0,diw,di(,di{,di",di[,dt<c>,df<c>
// CHANGE: cw,ciw,ci",ci(,ci{,cc,C
// CASE: ~ gUiw guiw
// VISUAL: v,V,Ctrl+q  vip,vap  ggVG
// COPY/PASTE: yy,yw,yiw,y$,y0,p,P  "ay "ap "+y "+p
// NUMBERS: Ctrl+a Ctrl+x
// UNDO/REDO: u,Ctrl+r,U
// FIND/REPLACE: :%s/old/new/g  :%s/old/new/gc
// REPEAT: . @<letter> @@
// LINE JOIN: J gJ
// INDENT: >ip <ip =ip
// ==================================================================
