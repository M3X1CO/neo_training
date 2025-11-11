# ===================================================================== 
# NEOVIM DAILY TRAINING - 12-15 MINUTE WORKOUT
# =====================================================================
# Custom bindings: j=UP, k=DOWN
# NOTE: u=undo throughout (mentioned once per exercise or less)
# ESC always returns to normal mode
# =====================================================================

# ma set a mark here by pressing ma 'a to jump back here
def exercise_a():
    # BASIC NAVIGATION - Learn to move around
    # k=down 5x to next line
    print("You made it here!")
    # j=up 1x to go back
    # k=down 3x to next line
    print("Now go back up!")
    # $=end of line, 0=start of line, l=right, h=left repeatedly
    print("Move through this line using l and h!")
    # 0=start, w=next word (try w w w to jump 3 words), b=back word, e=end of word
    print("If it works but you don't understand why, it's only a matter of time before it breaks.")
    # f and F jump to character: fI=jump forward to 'I', FI=jump backward to 'I'
    # t and T jump before character: tw=jump before 'w', Tw=jump backward before 'w'
    # ;=repeat last f/F/t/T, ,=repeat in opposite direction
    # 'a returns to mark

# mb
def exercise_b():
    # MARKS AND JUMPS - Set marks, they persist thru file closing
    first = 1  # 0 to start of line, mx to set mark x, k move down
    second = 2  # 0 to start of line, my to set mark y
    # 'b=jump to mark b, 'x=jump to mark x, 'y=jump to mark y
    # ''=jump to previous position
    # Press x to delete a char (any edit), k=down, `.=jump to last edit
    # mA-Z sets global marks. You can hop between two files
    # File mA and mB then 'A will take you to file A, 'B takes you to
    # file B fast tab through files your working
    # gg=top of file, 'b=back here, G=bottom of file, 'b=back
    # 10G=jump to line 10, :50 ENTER=jump to line 50, 'b=back
    # Ctrl+d=half page down, Ctrl+u=half page up
    # Ctrl+f=full page down, Ctrl+b=full page up
    # H=top of screen, M=middle of screen, L=bottom of screen
    # {=up one paragraph, }=down one paragraph
    # 'b returns to mark

# mc
def exercise_c():
    # SEARCH AND FIND - Locate text quickly
    new_delta = 42
    # Move onto "new_delta"
    # *=search forward, #=search back, n=next N=previous match
    new_delta_squared = new_delta * new_delta
    # search word /squared, ?squared ENTER forward-back n=next N=prev
    # :noh ENTER=  nooo.... no search highlighting
    # 'c returns to mark

# md
def exercise_d():
    # INSERT MODE and LINE JOINING
    print("Hello World")  # A=append at end, type !, ESC!
    # k=down to next line
    print("day one")
    # I=insert at start, type #, ESC (comments it), u=undo
    # o=open, O=open line below-above, type something, ESC, u=undo
    # Move to print line, gJ=join lines, J join w/space u=undo
    # 'd returns to mark

# me
def exercise_e():
    # DELETION - Basic and advanced text objects
    useless_variable = 123
    # Move to line above, dd=delete line, u=undo
    another_useless = 456
    # Move to line above, 0 then d$=delete to end, u=undo
    delete_me_please = 789
    # Move onto "delete_me_please", diw=delete inner word, u=undo
    # 0 then w then dw=delete first word, u=undo
    # $ then d0=delete end to start, u=undo
    # x=delete single char, u=undo
    values = (100, 200, 300)
    # Move to line, f(=jump to paren, di(=delete inside parens, u=undo
    # after deleting inside parens i to insert new text
    # da plus ( or [ or { or " deletes around including ([{", u=undo
    message = "delete the text inside these quotes"
    # Move to line, f"=jump to quote, di"=delete inside quotes, u=undo
    array = [1, 2, 3]
    # Move to line, f[=jump to [, di[=delete inside brackets, u=undo
    code = {"msg": "inside braces"}
    # Move to line, f{=jump to brace, di{=delete inside braces, u=undo
    sentence = "delete until semicolon; keep this"
    # w to first word, dt;=delete until ; (doesn't include it), u=undo
    # df;=delete including ;, u=undo
    with_spaces = "word here"
    # Move to "word", diw=delete inner word (just "word"), u=undo
    # 'e returns to mark

# mf
def exercise_f():
    # CHANGE COMMANDS and CASE CHANGES
    old_name = "change me"
    # Move to "old_name", ciw=change inner word. ESC, u=undo
    print("Replace this text")
    # Move to line, 0 then fR, cw=change word, type debug, ESC, u=undo
    # cc=change entire line, type new line, ESC, u=undo
    value = (100 + 200)
    # Move to line, f(=jump to (, ci(=change inside (), ESC, u=undo
    text = "modify this string"
    # Move to line, f"=jump to ", ci"=change inside "", ESC, u=undo
    # C=change to end of line, type something, ESC, u=undo
    UPPERCASE = "lowercase"
    # Move to "UPPERCASE", guiw=lowercase word, u=undo
    # Move to "lowercase", gUiw=uppercase word, u=undo
    # Move over first letter, ~=toggle case repeatedly
    # 'f returns to mark

# mg
def exercise_g():
    # VISUAL MODE - Select text character, line, and block-wise
    second_line = 2  # 0 then set mark mz here
    third_line = 3
    # 'z, V=visual line mode, k=extend down, d=delete, u=undo
    word1 = 1
    # fw, v=visual mode, e=extend to end, y=yank, k,0 P=paste, u=undo
    col1 = 10
    col2 = 20
    col3 = 30
    # www to =, Ctrl+q=visual block, k 2x, l 4x, d=delete, u=undo
    # SELECT ALL: ggVG gg=top, V=visual line, G=bottom, ESC=cancel
    # 'g returns to mark

# mh
def exercise_h():
    # << or >> tab an individual line back or forward
    # visual paragraph mode - select and fix entire paragraphs ¶

    badly_indented = 1
    another_bad = 2
    way_too_far = 3

    # move to line above, vip=select inner ¶, then = to auto-indent
    # vap=select around ¶, d=delete, u=undo

    misaligned_block = {
        "x": 1,
    "y": 2,
            "z": 3
    }

    # Move into block, vip>=indent right, u=undo
    # vip<=un-indent left, u=undo
    # vipy=yank paragraph, }=down ¶, p=paste, u=undo
    # 'h returns to mark

# mi
def exercise_i():
    # COPY AND PASTE - Yank with registers ® and clipboard
    copy_this_line = 999
    # yy=yank line, k=down, p=paste below, P=paste above, u=undo
    duplicate = "copy me"
    # Move to "duplicate", yiw=yank word, $, p=paste, u=undo
    register_practice = "use named registers"
    # "ayy=yank to ® a, k, "ap or "aP=paste below/above, u=undo
    word_one = 1
    word_two = 2
    # M2 "word_one", "byw=yank word ® b, k=down, diw "bP=paste, u=undo
    clipboard_test = "copy to system clipboard"
    # M2 line, "+yy=yank to clipboard, $, "+p
    # y$=yank to end, y0=yank to start
    # 'i returns to mark

# mj
def exercise_j():
    # EDIT OPERATIONS - Find/replace, undo/redo, increment/decrement
    new_list = [1, 2, 3]
    new_list_length = len(new_list)
    # :%s/new_list/old_list/g /gc replace all or w/check, u=undo
    # :s/new_list/old_list/ /g replace first in line or all, u=undo
    counter = 0
    # Move to line, f0=jump to 0, Ctrl+a=increment (0→1)
    # Ctrl+x=decrement (2→1), u=undo
    numbers = [10, 20, 30, 40]
    # M2L f1=jump to 10, Ctrl+a (10→11), f2 ^a f3 ^a U=undo all changes
    # 'j returns to mark

# mk
def exercise_k():
    # POWER MOVES - Macros, dot command, and speed combos
    item1 = 1
    item2 = 2
    # M2 item1, qq=record, ciw type element ESC, q=stop record
    # M2 item2, @q=replay macro, @@=replay last macro u=undo
    value1 = 10
    value2 = 20
    value3 = 30
    # M2 value1, ciw type number ESC, M2 value2, .=repeat last change
    # 'k returns to mark

# ====================================================================
# QUICK REFERENCE CHEAT SHEET
# ====================================================================
# MARKS: m<letter> '<letter> '' `.
# NAVIGATION: h,j(UP),k(DOWN),l  w,b,e  0,$  gg,G  {,}  H,M,L
#             f/F/t/T ;,  Ctrl+f/b Ctrl+d/u :N NG
# SEARCH: /text ?text n,N *,# :noh
# INSERT: i,a,I,A,o,O
# DELETE: x,dd,dw,d$,d0,diw,di(,di{,di",di[,dt<c>,df<c>
# CHANGE: cw,ciw,ci",ci(,ci{,cc,C
# CASE: ~ gUiw guiw
# VISUAL: v,V,Ctrl+q  vip,vap  ggVG
# COPY/PASTE: yy,yw,yiw,y$,y0,p,P  "ay "ap "+y "+p
# NUMBERS: Ctrl+a Ctrl+x
# UNDO/REDO: u,Ctrl+r,U
# FIND/REPLACE: :%s/old/new/g  :%s/old/new/gc
# REPEAT: . @<letter> @@
# LINE JOIN: J gJ
# INDENT: >ip <ip =ip
# ===================================================================
