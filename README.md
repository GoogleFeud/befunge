# Befunge

A fully functional [Befunge-93](https://esolangs.org/wiki/Befunge) interpreter written in Rust.

## Features

- Fully compatible
- Step by step interpretation
- Hooks for stack modification and the `p` operation
- Custom input/output methods

## Minor differences

- The grid is not limited to 80x25 spaces - the spaces here are unlimited BUT you'll have to add all the whitespaces yourself once you go past the default playfield.

## Some code to try out the interpreter with

(I haven't written any of these - all of em were found online)

### Tic-tac-toe

```
v123456789 --- >9 >48*,:55+\-0g,1v
>9>066+0p076+0p^  ^,," |"_v#%3:- <
:,,0537051v>:#,_$#^5#,5#+<>:#v_55+
74 1098709<^+55"---+---+---"0<v520
69 04560123 >:!#v_0\1v>$2-:6%v>803
6 +0g\66++0p^   $_>#%  v#9:-1_ 6/5
5  vv5!/*88\%*28 ::g0_^>9/#v_ "I",
,,5v>5++0p82*/3-:*+\:^v,_@ >"uoY",
0+5<v0+66_v#!%2:_55v  >:^:" win!"\
1-^ g   >$>0" :evom ruoY">:#,_$v>p
\*8+ 65_^#!/*88g0** `0\!`9:::<&<^0
v   >:!67+0g:!56+0g *+*+0" :evom "
>"yM">:#,_$ :. 1234+++, 789*+ \0^<
"a s't"98:*+>:#,_$@>365*+"ward"48*
 
```

### Calendar generator

**Enter the year you want to generate a calendar for in your terminal and press enter**

```
"P"00p&>:::4%!\"d"%*\45*:*%!+!!65*+31p:1-:::"I"5**\4/+\"d"/-\45*:*/+1+7%:0v
J!F?M!A M!J J!A!S O!N D!SaFrThWeTuMoSuvp01:_1#!-#%:#\>#+6<v-2g1+1g01p1p01:<
 January  February  March    April    >:45**00g\-\1-:v:<<6>+7%:10g2+:38*\`|
   May      June     July    August v02-1:+4*-4\`\4:/_$:^^:/*2+92+2:g00$$$<
September October  November December>p:45*+10g*\--2/00g4-2/>:#,1#*-#8\#4_$v
>20g>:#,1#*-#8\#4_$6"$S" v. .vp040\$_4#!8#\*#-,#1>#:<+5g03g01:,,:+55<0.p03<
^_v#:-1$_v#!:,*84,g1+1,< < ^ >::4%9*40g:8`!#v_$$$1+v^+g02_#v$#,$#+5<^_v#`\<
-#8\#4_$7>1-:2*64*+:1g>^ ^   ^,g+2/4\+p04+1:<>1#\-#<:66+\^ >>$10g30g>:#,1#*
> > $$55+,6>>40p:10g30g>:#,1#*-#8\#4_$>\:2*:1+1g2-50p1g640g-7*1+\-7v  v@,<6
2:+g01$_55+,^ > > > > #^>#g>#0>#2_v v*2!!\%+55:\/+55:**`0\!`g05:::\<  >2-^^
->:> >#^>#<>#<^#!:-1g04$$ < < < < < >4+8*+\:!!2*4+8*+,,48*,1+\1-:>#^_$$1+\1
```

### Cuboid

**Enter a width, height and depth in the following order**

```
"  :htdiW">:#,_>&>00p" :thgieH">:#,_>&>:10p0"  :htpeD">:#,_$>&>:20p55+,+:1`*:vv
v\-*`0:-g01\++*`\0:-\-1g01:\-*`0:-g02\+*`\0:-\-1g02<:::::<\g3`\g01:\1\+55\1-1_v
>":"\1\:20g\`!3g:30p\00g2*\::20g\`\20g1-\`+1+3g\1\30g\:20g-::0\`\2*1+*-\48*\:^v
/\_ @_\#!:!#$>#$_\#!:,#-\#1                         <+1\<*84g02"_"+1*2g00+551$<
```
