// ============================================================================ NEOVIM DAILY
// TRAINING - 15-20 MINUTE WORKOUT
// ============================================================================

// === EXERCISE 1: BASIC NAVIGATION (2 min) ===
// Current position: top of file
// Goal: Navigate to the println! below using ONLY these keys:
//   j = UP one line      k = DOWN one line   (your custom bindings!)
//   h = left one char    l = right one char
//   w = next word start  b = back word start
//   e = end of word

fn exercise_one() {
    println!("You made it here!");  // Press: 10k to jump down 10 lines
    println!("Now go back up!");    // Press: j to go up one line
    println!("Move word by word");  // Press: w w w to move 3 words right
}

// === EXERCISE 2: SUPER FAST NAVIGATION (3 min) ===
// Speed shortcuts:
//   gg = jump to top of file
//   G = jump to bottom of file
//   0 = jump to start of line
//   $ = jump to end of line
//   { = jump up one paragraph
//   } = jump down one paragraph
//   Ctrl+F = page down (full page)
//   Ctrl+B = page up (full page)
//   Ctrl+D = half page down
//   Ctrl+U = half page up
//   H = jump to top of visible screen
//   M = jump to middle of visible screen
//   L = jump to bottom of visible screen

fn exercise_two() {
    println!("Start here");
    // Press: $ to jump to end of this line
    // Press: 0 to jump back to start of line
    // Press: G to jump to bottom of file, then gg to come back to top
    // Press: Ctrl+D to jump down half a page
    // Press: Ctrl+U to jump back up half a page
    // Press: H to jump to top of screen, L for bottom, M for middle
}

// === EXERCISE 3: SEARCH AND FIND (3 min) ===
// Search commands:
//   /search_term = search forward for "search_term"
//   ?search_term = search backward
//   n = next match
//   N = previous match
//   * = search for word under cursor (forward)
//   # = search for word under cursor (backward)

fn exercise_three() {
    let new_delta = 42;
    let new_delta_squared = new_delta * new_delta;
    let new_delta_cubed = new_delta * new_delta * new_delta;
    
    // TASK: Place cursor on "new_delta" above and press *
    // This will highlight all instances. Press n to cycle through them.
    // Then press # to go backwards through matches.
}

// === EXERCISE 4: EDITING - INSERT MODE (2 min) ===
// Enter insert mode:
//   i = insert before cursor
//   a = insert after cursor
//   I = insert at start of line
//   A = insert at end of line
//   o = open new line below
//   O = open new line above
// Always press ESC to return to normal mode!

fn exercise_four() {
    println!("Hello World")  // Press: A then type ! then ESC
    // Press: o to create a new line below and type a new println
    // Press: O to create a new line above this comment
}

// === EXERCISE 5: DELETING TEXT (3 min) ===
// Delete commands (normal mode):
//   x = delete character under cursor
//   dd = delete entire line
//   dw = delete word
//   d$ = delete from cursor to end of line
//   d0 = delete from cursor to start of line
//   diw = delete inner word (cursor anywhere in word)
//   dap = delete a paragraph

fn exercise_five() {
    let useless_variable = 123;  // Press: dd to delete this entire line
    let another_useless = 456;   // Press: 0 then d$ to delete from start
    let delete_me_please = 789;  // Press: diw with cursor on "delete_me_please"
    
    // Practice: Delete the word "useless" above: 
    // 1. Navigate to "useless" (use /useless then ENTER)
    // 2. Press: diw (delete inner word)
    // 3. Press: u to undo
}

// === EXERCISE 6: CHANGE COMMANDS (3 min) ===
// Change = delete + enter insert mode:
//   cw = change word
//   ciw = change inner word (whole word)
//   ci" = change inside quotes
//   ci( = change inside parentheses
//   ci{ = change inside braces
//   cc = change entire line
//   C = change from cursor to end of line

fn exercise_six() {
    let old_name = "change me";
    println!("Replace this text");
    let value = (100 + 200);
    
    // TASK 1: Change "old_name" to "new_name"
    // 1. Put cursor on "old_name"
    // 2. Press: ciw (change inner word)
    // 3. Type: new_name
    // 4. Press: ESC
    
    // TASK 2: Change the string content
    // 1. Put cursor anywhere in "change me"
    // 2. Press: ci" (change inside quotes)
    // 3. Type new text
    // 4. Press: ESC
}

// === EXERCISE 7: VISUAL MODE - SELECT TEXT (3 min) ===
// Visual mode for selecting:
//   v = visual mode (character-wise)
//   V = visual line mode (line-wise)
//   Ctrl+v = visual block mode (column-wise)
// Once in visual mode:
//   Use h,j,k,l or w,b to expand selection
//   d = delete selection
//   c = change selection
//   y = yank (copy) selection

fn exercise_seven() {
    let first_line = 1;
    let second_line = 2;
    let third_line = 3;
    let fourth_line = 4;
    
    // TASK: Delete lines 2-3 above
    // 1. Navigate to "let second_line"
    // 2. Press: V (visual line mode)
    // 3. Press: k (extend selection down - remember your custom bindings!)
    // 4. Press: d (delete)
    // 5. Press: u (undo)
}

// === EXERCISE 8: COPY AND PASTE (3 min) ===
// Yank (copy) and paste:
//   yy = yank (copy) current line
//   yw = yank word
//   yiw = yank inner word
//   y$ = yank to end of line
//   p = paste after cursor
//   P = paste before cursor
//
// REGISTERS (named clipboards):
//   "ayy = yank line into register 'a'
//   "byw = yank word into register 'b'
//   "ap = paste from register 'a'
//   "bp = paste from register 'b'
//
// SYSTEM CLIPBOARD (copy/paste with other apps):
//   "+y = yank to system clipboard
//   "+p = paste from system clipboard
//   "+yy = yank line to clipboard
//   ggVG"+y = select all and copy to clipboard

fn exercise_eight() {
    let copy_this_line = 999;
    // TASK 1: Copy the line above and paste it below
    // 1. Navigate to the line above (press j with your bindings)
    // 2. Press: yy (yank line)
    // 3. Press: k (move down with your bindings)
    // 4. Press: p (paste below)
    
    let duplicate = "copy me";
    // TASK 2: Copy just the word "duplicate"
    // 1. Put cursor on "duplicate"
    // 2. Press: yiw (yank inner word)
    // 3. Navigate somewhere else
    // 4. Press: p to paste
    
    let test_register = "practice registers";
    // TASK 3: Use named registers
    // 1. Put cursor on the line above
    // 2. Press: "ayy (yank into register 'a')
    // 3. Navigate elsewhere
    // 4. Press: "ap (paste from register 'a')
    
    let clipboard_test = "copy to clipboard";
    // TASK 4: Copy to system clipboard
    // 1. Put cursor on line above
    // 2. Press: "+yy (yank to clipboard)
    // 3. Now you can Ctrl+V paste this in any other app!
    // 4. Press: "+p to paste from clipboard into neovim
}

// === EXERCISE 9: FIND AND REPLACE (3 min) ===
// Find and replace commands:
//   :s/old/new/        = replace first occurrence in line
//   :s/old/new/g       = replace all in line
//   :%s/old/new/g      = replace all in file
//   :%s/old/new/gc     = replace all with confirmation
//   :s/old/new/gc      = replace in line with confirmation

fn exercise_nine() {
    let new_list = vec![1, 2, 3];
    let new_list_length = new_list.len();
    let new_list_first = new_list[0];
    
    // TASK: Replace all "new_list" with "old_list"
    // 1. Press: :%s/new_list/old_list/g
    // 2. Press: ENTER
    // 3. Press: u to undo
    
    // Alternative: Replace with confirmation
    // 1. Press: :%s/new_list/old_list/gc
    // 2. Press: y (yes) or n (no) for each match
    // 3. Press: u to undo
}

// === EXERCISE 10: UNDO AND REDO (1 min) ===
// Time travel:
//   u = undo
//   Ctrl+r = redo
//   U = undo all changes on current line

fn exercise_ten() {
    let test = 123;
    
    // TASK: Practice undo/redo
    // 1. Delete this line with dd
    // 2. Press: u (undo)
    // 3. Delete it again with dd
    // 4. Press: Ctrl+r (redo - it deletes again)
    // 5. Press: u (undo - it comes back)
}

// === EXERCISE 11: ADVANCED DELETION (2 min) ===
// Power delete commands:
//   di( = delete inside parentheses
//   di{ = delete inside braces
//   di[ = delete inside brackets
//   di" = delete inside quotes
//   da( = delete around parentheses (includes parens)
//   dt<char> = delete until <char>
//   df<char> = delete including <char>

fn exercise_eleven() {
    let values = (100, 200, 300);
    let message = "delete the text inside these quotes";
    let array = [1, 2, 3, 4, 5];
    let code = { println!("inside braces"); };
    
    // TASK 1: Delete content inside parentheses
    // 1. Cursor anywhere in (100, 200, 300)
    // 2. Press: di(
    
    // TASK 2: Delete the text inside quotes
    // 1. Cursor anywhere in the string above
    // 2. Press: di"
    
    // TASK 3: Delete until the semicolon
    // 1. Cursor at start of a line (press 0)
    // 2. Press: dt;
}

// === EXERCISE 12: MARKS AND JUMPS (2 min) ===
// Bookmarks (marks):
//   m<letter> = set mark (e.g., ma, mb, mc)
//   '<letter> = jump to mark (e.g., 'a, 'b, 'c)
//   '' = jump back to previous position
//   `. = jump to last edit position

fn exercise_twelve() {
    // Line 1: Set a mark here
    let first = 1;
    
    // Line 2: Set another mark here
    let second = 2;
    
    // TASK: Practice marks
    // 1. Navigate to Line 1
    // 2. Press: ma (set mark 'a')
    // 3. Navigate to Line 2
    // 4. Press: mb (set mark 'b')
    // 5. Press: 'a to jump to mark a
    // 6. Press: 'b to jump to mark b
}

// === EXERCISE 13: MACROS - ULTIMATE POWER (2 min) ===
// Record and replay actions:
//   q<letter> = start recording macro (e.g., qa)
//   q = stop recording
//   @<letter> = play macro (e.g., @a)
//   @@ = replay last macro

fn exercise_thirteen() {
    let item1 = 1;
    let item2 = 2;
    let item3 = 3;
    
    // TASK: Record a macro to delete "item" and replace with "element"
    // 1. Navigate to the first "item" above
    // 2. Press: qa (start recording to register 'a')
    // 3. Press: ciw (change inner word)
    // 4. Type: element
    // 5. Press: ESC
    // 6. Press: q (stop recording)
    // 7. Move to next line with k (down with your bindings)
    // 8. Press: @a (replay macro)
    // 9. Press: @@ (replay last macro on item3)
}

// === EXERCISE 14: SELECT ALL AND DELETE (1 min) ===
// Nuclear option:
//   ggVG = select entire file (gg + V + G)
//   ggdG = delete entire file (gg + d + G)

fn exercise_fourteen() {
    // TASK: Select all and delete (then undo!)
    // 1. Press: gg (go to top)
    // 2. Press: V (visual line mode)
    // 3. Press: G (select to bottom)
    // 4. Press: d (delete)
    // 5. Press: u (UNDO IMMEDIATELY!)
    
    // OR shortcut:
    // 1. Press: ggdG (delete entire file in one command)
    // 2. Press: u (UNDO!)
}

// === EXERCISE 15: SPEED COMBOS (2 min) ===
// Powerful combinations to practice:
//   ci" = change inside quotes
//   di{ = delete inside braces
//   yap = yank a paragraph
//   dap = delete a paragraph
//   >ip = indent paragraph
//   <ip = unindent paragraph
//   gqip = format/wrap paragraph format text to 80 char length.
//   gq}  = format until the end of the paragraph.
//   gqap = format around the paragraph (includes any blank lines).
//   gggqG = format the entire file.
//   :set textwidth=80

fn exercise_fifteen() {
    let config = {
        name: "test",
        value: 123,
        enabled: true
    };
    
    // TASK: Delete everything inside the braces
    // 1. Cursor anywhere in the config
    // 2. Press: di{
    // 3. Press: u to undo
}

// ============================================================================
// === BONUS: DAILY PRACTICE ROUTINE ===
// ============================================================================
// Repeat these tasks daily:
// 1. Navigate to a function using /fn
// 2. Change a variable name using ciw
// 3. Delete a line with dd
// 4. Copy a line with yy and paste with p
// 5. Find and replace a word using :%s/old/new/g
// 6. Delete content inside quotes/braces using di" or di{
// 7. Select multiple lines with V and delete with d
// 8. Undo and redo with u and Ctrl+r
// 9. Copy to clipboard with "+yy and paste with "+p
//
// TIP: Never reach for the mouse! Every time you want to use the mouse,
//      stop and figure out the keyboard way. This is how you build muscle memory.
// ============================================================================

fn main() {
    println!("Neovim Training Program");
    println!("Complete all exercises above!");
    println!("Remember: ESC returns to normal mode");
    println!("Practice daily for 15-20 minutes");
    println!("You'll be a wizard in no time!");
}

// ============================================================================
// QUICK REFERENCE CHEAT SHEET
// ============================================================================
// NAVIGATION: h,j(UP),k(DOWN),l  w,b,e  0,$  gg,G  {,}
//             Ctrl+F(page down) Ctrl+B(page up) Ctrl+D(half down) Ctrl+U(half up)
//             H(top screen) M(middle screen) L(bottom screen)
// SEARCH: /text  n,N  *,#
// INSERT: i,a,I,A,o,O
// DELETE: x,dd,dw,d$,diw,di(,di{,di"
// CHANGE: cw,ciw,ci",ci(,ci{,cc,C
// VISUAL: v,V,Ctrl+v
// COPY/PASTE: yy,yw,yiw,p,P
// REGISTERS: "ayy "byw "ap "bp (named clipboards)
// CLIPBOARD: "+y (copy to system) "+p (paste from system) ggVG"+y (copy all)
// UNDO/REDO: u,Ctrl+r
// FIND/REPLACE: :%s/old/new/g
// MACROS: q<letter> [actions] q, then @<letter>
// ============================================================================
