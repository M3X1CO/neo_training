# ==================================================================
# neovim daily training - 12-15 minute workout
# ==================================================================
# custom bindings: j=up, k=down
# note:always start by setting a mark before each exercise
# move to the first line of code(not function) then follow the
# instruction below.
# after every change you make if in insert mode press esc u to undo
# ==================================================================

# ma set a mark here by pressing ma 'a to jump back here
def exercise_a():  # basic navigation - learn to move around
    # k=down 4x to next line
    print("you made it here!")
    # j=up 1x to go back
    # k=down 3x to next line
    print("now go back up!")
    # $=end of line, 0=start of line, l=right, h=left repeatedly
    print("move through this line using l and h!")
    # w=next word, b=back word, e=end of word
    print("if it works but you don't understand why, "
          "it's only a matter of time before it breaks.")
    # f and f jump to char: fi=jump forward to 'i', fi= back to 'i'
    # t and t jump before char: tw=before 'w', tw=back before 'w'
    # ;=repeat last f/f/t/t, ,=repeat in opposite direction
    # 'a returns to mark


# mb
def exercise_b():  # marks and jumps - persist thru file closing
    first = 1  # 0 to start, mx to set mark x, k move down
    second = 2  # 0 to start, my to set mark y
    # 'b=jump to mark b, 'x=jump to mark x, 'y=jump to mark y
    # ''=jump to previous position
    # press x to delete a char, k=down, `.=jump to last made edit
    # ma-z sets global marks. you can hop between two files
    # file ma and mb then 'a will take you to file a, 'b takes you
    # to file b fast tab through files your working
    # gg=top of file, 'b=back here, g=bottom of file, 'b=back
    # 10g=jump to line 10, :50 enter=jump to line 50, 'b
    # ctrl+d=half page down, ctrl+u=half page up
    # ctrl+f=full page down, ctrl+b=full page up
    # h=top of screen, m=middle of screen, l=bottom of screen
    # {=up one paragraph ¶, }=down one paragraph ¶
    # 'b returns to mark


# mc
def exercise_c():  # search and find - locate text quickly
    new_delta = 42
    # move onto "new_delta"
    # *=search forward, #=search back, n=next n=previous match
    new_delta_squared = new_delta * new_delta
    # search word /squared, ?squared enter forward-back n=next n=prev
    # :noh enter=  nooo.... no search highlighting
    # 'c returns to mark


# md
def exercise_d():  # insert mode and line joining
    print
    ("hello world"
    # move to hello world, a=append at end, type ), esc u=undo
    # j=up a line gj=join lines, j join w/space
    print("day one")
    # i=insert at start, type # , esc (comments it)
    # o=open, o=open line below-above, type something
    # 'd returns to mark


# me
def exercise_e():  # deletion - basic and advanced text objects
    useless_variable = 123
    # dd=delete line u=undo
    another_useless = 456
    # 0 then d$=delete to end
    delete_me_please = 789
    # move onto "delete_me_please", diw=delete inner word
    # 0 then w then dw=delete first word
    # $ then d0=delete end to start
    # x=delete single char
    values = (100, 200, 300)
    # f(=jump to paren, di(=delete inside parens
    # da plus ( or [ or { or " deletes around ([{"
    message = "delete the text inside these quotes"
    # f"=jump to quote, di"=delete inside ""
    array = [1, 2, 3]
    # f[=jump to [, di[=delete inside []
    code = {"key": print("inside braces")}
    # f{=jump to brace, di{=delete inside {}
    sentence = "delete until semicolon; keep this"
    # dt;=delete up to ;
    # df;=delete including ;
    # 'e returns to mark


# mf
def exercise_f():  # change commands and case changes
    old_name = "change me"
    # move to "old_name", ciw=change inner word. esc, u=undo
    print("replace this text")
    # 0 then fr, cw=change word, type debug
    # cc=change entire line, type new line
    value = (100, 200)
    # f(=jump to (, ci(=change inside ()
    text = "modify this string"
    # f"=jump to ", ci"=change inside ""
    # c=change to end of line, type something
    uppercase = "lowercase"
    # fu, guiw=lowercase word
    # flfl, guiw=uppercase word
    # move over any letter, ~=toggle case repeatedly
    # 'f returns to mark


# mg
def exercise_g():  # visual mode - select char, line, and block
    second_line = 2  # 0 then set mark mz here
    third_line = 3
    # 'z, v=visual line mode, k=extend down, d=delete
    word1 = 1

    # fw, v=visual mode, e=extend to end, y=yank, k p=paste
    col1 = 10
    col2 = 20
    col3 = 30
    # www to =, ctrl+q=visual block, k 2x, l 4x, d=delete
    # select all: ggvg gg=top, v=visual line, g=bottom
    # 'g returns to mark


# mh
        def exercise_h():  # formatting
    # << or >> tab an individual line back or forward

badly_indented = 1
another_bad = 2
way_too_far = 3

    # vip=select inner ¶, then = to auto-indent
    # vap=select around includes trailing space ¶, d=delete

        misaligned_block = {
            "x": 1,
        "y": 2,
                "z": 3
        }

    # vip>=indent right
    # vip<=un-indent left
    # vipy=yank paragraph, }=down ¶, p=paste
    # 'h returns to mark


# mi
def exercise_i():  # copy and paste - yank w/ registers ® and clipboard
    copy_this_line = 999
    # yy=yank line, k=down, p=paste below, P=paste above
    duplicate = "copy me"
    # move to "duplicate", yiw=yank word, $, p=paste
    register_practice = "use named registers"
    # "ayy=yank to ® a, k, "ap or "aP=paste below/above
    word_one = 1
    word_two = 2
    # m2 "word_one", "byw=yank word ® b, k=down, diw "bp=paste
    clipboard_test = "copy to system clipboard"
    # "+yy=yank to clipboard, "+p /p paste below/above
    # y$=yank to end, y0=yank to start
    # 'i returns to mark


# mj
def exercise_j():  # edit operations - find/replace, increment/decrement
    new_list = [1, 2, 3]
    new_list_length = len(new_list)
    # :%s/new_list/old_list/g /gc replace all or w/check
    # :s/new_list/old_list/ /g replace first in line or all
    counter = 0
    # f0=jump to 0, ctrl+a=increment (0→1)
    # ctrl+x=decrement (2→1)
    numbers = [10, 20, 30, 40]
    # f1=jump to 10, ctrl+a (10→11), f2 ^a f3 ^a u=undo all changes
    # 'j returns to mark


# mk
def exercise_k():  # power moves - macros, dot command
    item1 = 1
    item2 = 2
    # m2 item1, qq=record, ciw type element esc, q=stop record
    # m2 item2, @q=replay macro, @@=replay last macro
    value1 = 10
    value2 = 20
    value3 = 30
    # m2 value1, ciw type number esc, m2 value2, .=repeat last change
    # 'k returns to mark


if __name__ == "__main__":
    pass

# ==================================================================
# quick reference cheat sheet
# ==================================================================
# marks: m<letter> '<letter> '' `.
# navigation: h,j(up),k(down),l  w,b,e  0,$  gg,g  {,}  h,m,l
#             f/f/t/t ;,  ctrl+f/b ctrl+d/u :n ng
# search: /text ?text n,n *,# :noh
# insert: i,a,i,a,o,o
# delete: x,dd,dw,d$,d0,diw,di(,di{,di",di[,dt<c>,df<c>
# change: cw,ciw,ci",ci(,ci{,cc,c
# case: ~ guiw guiw
# visual: v,v,ctrl+q  vip,vap  ggvg
# copy/paste: yy,yw,yiw,y$,y0,p,p  "ay "ap "+y "+p
# numbers: ctrl+a ctrl+x
# undo/redo: u,ctrl+r,u
# find/replace: :%s/old/new/g  :%s/old/new/gc
# repeat: . @<letter> @@
# line join: j gj
# indent: >ip <ip =ip
# ==================================================================
