# ===================================================================== 
# NEOVIM DAILY TRAINING - 12-15 MINUTE WORKOUT (Python Version)
# =====================================================================
# START HERE: Begin with exercise_a at line 16, set mark with ma
# Custom bindings: j=UP, k=DOWN
# NOTE: u=undo throughout (mentioned once per exercise or less)
# ESC always returns to normal mode
# =====================================================================

# ma
def exercise_a():
    # BASIC NAVIGATION - Learn to move around
    # k=down 5x to next line
    print("You made it here!")
    # j=up 1x to go back
    # k=down 2x to next line
    print("Now go back up!")
    # 0=start of line, then l=right repeatedly, $=end of line, h=left repeatedly
    print("Move through this line using l and h!")
    # 0=start, w=next word (try w w w to jump 3 words), b=back word, e=end of word
    print("If it works but you don't understand why, it's only a matter of time before it breaks.")
    # f and F jump to character: fI=jump forward to 'I', FI=jump backward to 'I'
    # t and T jump before character: tw=jump before 'w', Tw=jump backward before 'w'
    # ;=repeat last f/F/t/T, ,=repeat in opposite direction
    # 'a returns to mark

# mb
def exercise_b():
    # MARKS AND JUMPS - Set bookmarks to jump around
    first = 1
    # k=down 2x, my=set mark y
    second = 2
    # 'b=jump to mark b, 'x=jump to mark x, 'y=jump to mark y
    # ''=jump to previous position
    # Press x to delete a char (any edit), k=down, `.=jump to last edit
    # 'b returns to mark

# mc
def exercise_c():
    # SUPER FAST NAVIGATION - Jump across file and screen
    print("Start here")
    # $=end of line, 0=start of line
    # gg=top of file, 'c=back here, G=bottom of file, 'c=back
    # 10G=jump to line 10, :50 ENTER=jump to line 50, 'c=back
    # Ctrl+d=half page down, Ctrl+u=half page up
    # Ctrl+f=full page down, Ctrl+b=full page up
    # H=top of screen, M=middle of screen, L=bottom of screen
    # {=up one paragraph, }=down one paragraph
    # 'c returns to mark

# md
def exercise_d():
    # SEARCH AND FIND - Locate text quickly
    new_delta = 42
    new_delta_squared = new_delta * new_delta
    new_delta_cubed = new_delta * new_delta * new_delta
    # *=search forward, n=next match, N=previous match
    # #=search backward for word under cursor
    # /squared ENTER=search forward for "squared", n=next match
    # ?cubed ENTER=search backward for "cubed"
    # :noh ENTER=remove search highlighting
    # 'd returns to mark

# me
def exercise_e():
    # INSERT MODE and LINE JOINING
    print("Hello World")  # A=append at end, type !, ESC
    print("day one")
    message = """Split
across lines"""
    # Move to "Hello World!" line, k=down, J=join lines (adds space), u=undo
    # gJ=join without space, u=undo
    # 'e returns to mark

# mf
def exercise_f():
    # DELETION - Basic and advanced text objects
    useless_variable = 123
    another_useless = 456
    delete_me_please = 789
    values = (100, 200, 300)
    message = "delete the text inside these quotes"
    array = [1, 2, 3]
    code = {"msg": "inside braces"}
    sentence = "delete until semicolon; keep this"
    with_spaces = "word here"
    # Various deletion commands explained in comments
    # 'f returns to mark

# mg
def exercise_g():
    # CHANGE COMMANDS and CASE CHANGES
    old_name = "change me"
    print("Replace this text")
    value = 100 + 200
    text = "modify this string"
    UPPERCASE = "lowercase"
    # Move to "UPPERCASE", guiw=lowercase word, u=undo
    # Move to "lowercase", gUiw=uppercase word, u=undo
    # 'g returns to mark

# mh
def exercise_h():
    # VISUAL MODE - Select text character, line, and block-wise
    first_line, second_line, third_line, fourth_line = 1, 2, 3, 4
    word1, word2 = 1, 2
    col1, col2, col3 = 10, 20, 30
    # Visual block and line selections
    # Nuclear option: ggdG=delete entire file, u IMMEDIATELY
    # 'h returns to mark

# mi
def exercise_i():
    # VISUAL PARAGRAPH MODE - Select and fix entire paragraphs
    badly_indented = 1
    another_bad = 2
    way_too_far = 3
    misaligned_block = {"x":1, "y":2, "z":3}
    paragraph_one = "This is a paragraph."
    paragraph_two = "Another paragraph here."
    paragraph_three = "Yet another paragraph."
    paragraph_four = "Final paragraph."
    # vip=select inner paragraph, vap=select around paragraph
    # 'i returns to mark

# mj
def exercise_j():
    # COPY AND PASTE - Yank with registers and clipboard
    copy_this_line = 999
    duplicate = "copy me"
    register_practice = "use named registers"
    word_one, word_two = 1, 2
    clipboard_test = "copy to system clipboard"
    # "+yy=yank to clipboard, "+p=paste
    # 'j returns to mark

# mk
def exercise_k():
    # EDIT OPERATIONS - Find/replace, undo/redo, increment/decrement
    new_list = [1,2,3]
    new_list_length = len(new_list)
    new_list_first = new_list[0]
    test = 123
    counter = 0
    numbers = [10,20,30,40]
    # :%s/old/new/g  dd, u, Ctrl+r, Ctrl+a, Ctrl+x
    # 'k returns to mark

# ml
def exercise_l():
    # POWER MOVES - Macros, dot command, and speed combos
    item1, item2, item3 = 1,2,3
    value1, value2, value3 = 10,20,30
    config = {"name":"test", "value":123, "enabled":True}
    quoted = "change this text"
    paragraph_text = "This is a long paragraph that needs formatting."
    indented, needs_indent = 1,2
    # >ip=indent right, <ip=un-indent left, gqip=format paragraph, gggqG=format file
    # 'l returns to mark

# =======================================================================
# QUICK REFERENCE CHEAT SHEET
# =======================================================================
# MARKS: m<letter> '<letter> '' `.
# NAVIGATION: h,j(UP),k(DOWN),l w,b,e 0,$ gg,G {,} H,M,L
# SEARCH: /text ?text n,N *,# :noh
# INSERT: i,a,I,A,o,O
# DELETE: x,dd,dw,d$,d0,diw,daw,di(,di{,di",di[,dt<c>,df<c>,dap
# CHANGE: cw,ciw,ci",ci(,ci{,cc,C
# CASE: ~ gUiw guiw
# VISUAL: v,V,Ctrl+v vip,vap ggVG
# COPY/PASTE: yy,yw,yiw,y$,y0,p,P "ay "ap "+y "+p
# NUMBERS: Ctrl+a Ctrl+x
# UNDO/REDO: u,Ctrl+r,U
# FIND/REPLACE: :%s/old/new/g :%s/old/new/gc
# REPEAT: . @<letter> @@
# LINE JOIN: J gJ
# INDENT: >ip <ip =ip
# FORMAT: gqip gggqG
# =======================================================================

