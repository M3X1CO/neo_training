// ===================================================================== 
// NEOVIM DAILY TRAINING - 12-15 MINUTE WORKOUT
// =====================================================================
// Custom bindings: j=UP, k=DOWN
// NOTE: u=undo throughout (mentioned once per exercise or less)
// ESC always returns to normal mode
// =====================================================================

// ma set a mark here by pressing ma 'a to jump back here
fn exercise_a() {
    // BASIC NAVIGATION - Learn to move around
    // k=down 5x to next line
    println!("You made it here!");
    // j=up 1x to go back
    // k=down 3x to next line
    println!("Now go back up!");
    // $=end of line, 0=start of line, l=right repeatedly, h=left repeatedly
    println!("Move through this line using l and h!");
    // 0=start, w=next word (try w w w to jump 3 words), b=back word, e=end of word
    println!("If it works but you don't understand why, it's only a matter of time before it breaks.");
    // f and F jump to character: fI=jump forward to 'I', FI=jump backward to 'I'
    // t and T jump before character: tw=jump before 'w', Tw=jump backward before 'w'
    // ;=repeat last f/F/t/T, ,=repeat in opposite direction
    // 'a returns to mark
}

// mb
fn exercise_b() {
    // MARKS AND JUMPS - Set bookmarks to jump around, marks persist thru file closing
    let first = 1; // 0 to start of line, mx to set mark x
    let second = 2; // 0 to start of line, my to set mark y
    // 'b=jump to mark b, 'x=jump to mark x, 'y=jump to mark y
    // ''=jump to previous position
    // Press x to delete a char (any edit), k=down, `.=jump to last edit
    // mA-Z sets global marks. You can hop between two files by setting a mark in
    // each file mA and mB for ex. then 'A will take you to file A, 'B takes you to
    // file B fast tab through files your working
    // gg=top of file, 'b=back here, G=bottom of file, 'b=back
    // 10G=jump to line 10, :50 ENTER=jump to line 50, 'b=back
    // Ctrl+d=half page down, Ctrl+u=half page up
    // Ctrl+f=full page down, Ctrl+b=full page up
    // H=top of screen, M=middle of screen, L=bottom of screen
    // {=up one paragraph, }=down one paragraph
    // 'b returns to mark
}

// mc
fn exercise_c() {
    // SEARCH AND FIND - Locate text quickly
    let new_delta = 42;
    // Move onto "new_delta", *=search forward, n=next match, N=previous match
    // #=search backward for word under cursor
    let new_delta_squared = new_delta * new_delta;
    // /squared ENTER=search forward for "squared", n=next match
    let new_delta_cubed = new_delta * new_delta * new_delta;
    // ?cubed ENTER=search backward for "cubed"
    // :noh ENTER=no search highlighting
    // 'c returns to mark
}

// md
fn exercise_d() {
    // INSERT MODE and LINE JOINING
    println!("Hello World")  // A=append at end, type !, ESC
    // k=down to next line
    println!("day one")
    // I=insert at start, type //, ESC (comments it), u=undo
    // o=open line below, type something, ESC, u=undo
    // O=open line above, type something, ESC, u=undo
    // Move to "Hello World!" line, k=down, J=join lines (adds space), u=undo
    // gJ=join without space, u=undo
    let message = "Split across lines";
    // Move into string, J=join lines
    // 'd returns to mark
}

// me
fn exercise_e() {
    // DELETION - Basic and advanced text objects
    // On these exercises its best to move to line then 0 to start at beginning
    let useless_variable = 123;
    // Move to line above, dd=delete line, u=undo
    let another_useless = 456;
    // Move to line above, 0 then d$=delete to end, u=undo
    let delete_me_please = 789;
    // Move onto "delete_me_please", diw=delete inner word, u=undo
    // 0 then w then dw=delete first word, u=undo
    // $ then d0=delete end to start, u=undo
    // x=delete single char, u=undo
    let values = (100, 200, 300);
    // Move to line, f(=jump to paren, di(=delete inside parens, u=undo
    // after deleting inside parens i to insert new text
    // da(=delete around (includes parens), u=undo
    let message = "delete the text inside these quotes";
    // Move to line, f"=jump to quote, di"=delete inside quotes, u=undo
    let array = [1, 2, 3];
    // Move to line, f[=jump to bracket, di[=delete inside brackets, u=undo
    let code = { println!("inside braces"); };
    // Move to line, f{=jump to brace, di{=delete inside braces, u=undo
    let sentence = "delete until semicolon; keep this";
    // Move to line, 0=start, dt;=delete until ; (doesn't include it), u=undo
    // df;=delete including ;, u=undo
    let with_spaces = "word here";
    // Move to "word", diw=delete inner word (just "word"), u=undo
    // 'e returns to mark
}

// mf
fn exercise_f() {
    // CHANGE COMMANDS and CASE CHANGES
    let old_name = "change me";
    // Move to "old_name", ciw=change inner word, type new_name, ESC, u=undo
    println!("Replace this text");
    // Move to line above, 0 then fR, cw=change word, type debug, ESC, u=undo
    // Move to line, cc=change entire line, type new line, ESC, u=undo
    let value = (100 + 200);
    // Move to line, f(=jump to paren, ci(=change inside parens, type 50 + 50, ESC, u=undo
    let text = "modify this string";
    // Move to line, f"=jump to quote, ci"=change inside quotes, type new text, ESC, u=undo
    // C=change to end of line, type something, ESC, u=undo
    let UPPERCASE = "lowercase";
    // Move to "UPPERCASE", guiw=lowercase word, u=undo
    // Move to "lowercase", gUiw=uppercase word, u=undo
    // Move over first letter, ~=toggle case repeatedly
    // 'f returns to mark
}

// mg
fn exercise_g() {
    // VISUAL MODE - Select text character, line, and block-wise
    let second_line = 2;
    let third_line = 3;
    let fourth_line = 4;
    // Move to "let second_line", V=visual line mode, k=extend down, d=delete, u=undo
    let word1 = 1;
    // Move to "word1", v=visual mode, e=extend to end, y=yank, p=paste, u=undo
    let col1 = 10;
    let col2 = 20;
    let col3 = 30;
    // Move to first =, Ctrl+q=visual block, k 2x=down, l 4x=right, d=delete, u=undo
    // SELECT ALL: gg=top, V=visual line, G=bottom (entire file), ESC=cancel
    // 'g returns to mark
}

// mh
        fn exercise_h() {
    // << or >> tab an individual line back or forward
    // VISUAL PARAGRAPH MODE - Select and fix entire paragraphs
    // This function is intentionally misaligned!
    
    let badly_indented = 1;
    let another_bad = 2;
    let way_too_far = 3;
    
    // Move to var 1 above, vip=select inner paragraph (stays in visual mode)
    // While selected, press = which auto-indents to fix, u=undo
    // vap=select around paragraph (includes blank lines), d=delete, u=undo
    
    let misaligned_block = {
        let x = 1;
    let y = 2;
            let z = 3;
    };
    
    // Move into block, vip>=indent right (> while in visual mode), u=undo
    // vip<=un-indent left, u=undo
    // vipy=yank paragraph, k=down, p=paste, u=undo
    
    let paragraph_one = "This is a paragraph.";
    let paragraph_two = "Another paragraph here.";
    
    let paragraph_three = "Yet another paragraph.";
    let paragraph_four = "Final paragraph.";
    
    // Move to first block, vip=select inner (excludes blank), ESC=cancel
    // vap=select around (includes blank), ESC=cancel
    // 'h returns to mark
}

// mi
fn exercise_i() {
    // COPY AND PASTE - Yank with registers and clipboard
    let copy_this_line = 999;
    // Move to line above, yy=yank line, k=down, p=paste below, P=paste above, u=undo
    let duplicate = "copy me";
    // Move to "duplicate", yiw=yank word, k=down, move somewhere, p=paste, u=undo
   let register_practice = "use named registers";
    // Move to line, "ayy=yank to register a, k=down, "ap=paste from register a, u=undo
    let word_one = 1;
    let word_two = 2;
    // Move to "word_one", "byw=yank word to register b, k=down to "word_two", diw "bp=paste, u=undo
    let clipboard_test = "copy to system clipboard";
    // Move to line, "+yy=yank to clipboard (now Ctrl+V works in other apps!)
    // "+p=paste from clipboard, u=undo
    // y$=yank to end, y0=yank to start
    // 'i returns to mark
}

// mj
fn exercise_j() {
    // EDIT OPERATIONS - Find/replace, undo/redo, increment/decrement
    let new_list = vec![1, 2, 3];
    let new_list_length = new_list.len();
    let new_list_first = new_list[0];
    // :%s/new_list/old_list/g ENTER=replace all in file, u=undo
    // :%s/new_list/old_list/gc ENTER=replace with confirmation (y/n), u=undo
    // :s/new_list/old_list/ ENTER=replace first in line, u=undo
    // :s/new_list/old_list/g ENTER=replace all in line, u=undo
    let counter = 0;
    // Move to line, f0=jump to 0, Ctrl+a=increment (0→1), Ctrl+a again (1→2)
    // Ctrl+x=decrement (2→1), u=undo all
    let numbers = [10, 20, 30, 40];
    // Move to line, f1=jump to 10, Ctrl+a (10→11), ww=next number, Ctrl+a (20→21)
    // u=undo, U=undo all changes on line
    // 'j returns to mark
}

// mk
fn exercise_k() {
    // POWER MOVES - Macros, dot command, and speed combos
    let item1 = 1;
    let item2 = 2;
    let item3 = 3;
    // Move to "item1", qq=start recording, ciw type element ESC, q=stop recording
    // Move to "item2", @q=replay macro, move to "item3", @@=replay last, u u u=undo all
    let value1 = 10;
    let value2 = 20;
    let value3 = 30;
    // Move to "value1", ciw type number ESC, move to "value2", .=repeat last change
    // Move to "value3", .=repeat again, u u u=undo all
    let quoted = "change this text";
    // Move to line, ci"=change inside quotes, type new text, ESC, u=undo

    // gqip=format/wrap paragraph, u=undo
    // gggqG=format entire file
    // 'k returns to mark
}

// ============================================================================
// QUICK REFERENCE CHEAT SHEET
// ============================================================================
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
// FORMAT: gqip gggqG
// ============================================================================
