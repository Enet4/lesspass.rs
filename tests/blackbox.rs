//! Tests lesspass.rs using lesspass-cli as oracle.
//!
//! The test vectors in this file are generated by lesspass-cli using
//! the script `make-blackbox-tests.sh` located in the same directory
//! as this test.
use lesspass::*;

pub fn generate(website: &str, login: &str, password: &str, counter: u8,
                length: u8, charset: CharacterSet) -> String {
    let salt = generate_salt(website, login, counter);
    let entropy = generate_entropy(password, &salt,
                                   Algorithm::SHA256,
                                   100000);
    render_password(&entropy, charset, length)
}

fn t(n: usize, website: &str, login: &str, password: &str, counter: u8,
     length: u8, passwords: &[&str]) {
    // Running the key derivation function in debug builds is really
    // slow.  Only run the first four test functions on the assumption
    // that most systems nowadays have at least four cores.
    if cfg!(debug_assertions) && n > 3 {
        eprintln!("Skipping test {} in non-optimized build.", n);
        return;
    }

    let mut i = 0;
    for &lower in &[false, true] {
        for &upper in &[false, true] {
            for &digits in &[false, true] {
                for &symbols in &[false, true] {
                    if !lower && !upper && !digits && !symbols {
                        continue;
                    }
                    // Start with the empty set.
                    let mut charset =
                        CharacterSet::Lowercase
                        & !CharacterSet::Lowercase;

                    if lower {
                        charset |= CharacterSet::Lowercase;
                    }
                    if upper {
                        charset |= CharacterSet::Uppercase;
                    }
                    if digits {
                        charset |= CharacterSet::Numbers;
                    }
                    if symbols {
                        charset |= CharacterSet::Symbols;
                    }
                    assert_eq!(
                        generate(website, login, password, counter, length,
                                 charset),
                        passwords[i],
                        "Mismatch for website: {:?}, login: {:?}, \
                         password: {:?}, counter: {:?}, length: {:?}, \
                         lower: {:?}, upper: {:?}, digits: {:?}, \
                         symbols: {:?}",
                        website, login, password, counter, length,
                        lower, upper, digits, symbols);
                    i += 1;
                }
            }
        }
    }
}

#[test]
fn vectors_00() {
    t(0, "example.org", "user@example.org", "password", 1, 12, &[".;#+]<`.:.^[", "565301097089", "@95:&/=`91.,", "LFOTFIGTSMDD", "FXZ,LY:::WF.", "ZZDN0999N51D", "\"0C6!6O;6P&9", "lfotfigtsmdd", "fxz,ly:::wf.", "zzdn0999n51d", "\"0c6!6o;6p&9", "LcKfxZFrkJzv", "@.`-JuB_+_uq", "BdNBGqC3wMmO", "X%W(PxH#j7_2", ]);
}

#[test]
fn vectors_01() {
    t(1, "example.org", "user@example.org", "password", 1, 16, &[".;>#+]<`.:.[^&#!", "5653901097890787", "@$5:&/=`91,(3911", "LFOTMFITSMDDGWYP", "XZ,~LYH::WF.:.:D", "ZZDN0999H5D5AQ9B", "1\"0U6!6OP&9'R>;R", "lfotmfitsmddgwyp", "xz,~lyh::wf.:.:d", "zzdn0999h5d5aq9b", "1\"0u6!6op&9'r>;r", "LXcKfxZFrzzvKeQz", "@'.`-wB_+Iuqgu&y", "BNGqCpwM7MmO3Pd1", "(XW(Px#_z27Dk>5a", ]);
}

#[test]
fn vectors_02() {
    t(2, "example.org", "user@example.org", "password", 2, 12, &["!<|}-]=)#\"~\"", "565035760301", "?49=&<!'88((", "EWBMSQPYUNSN", "M_$<};A\"HS-*", "7M7TJD7D2740", "U(K2:5{'$K{T", "ewbmsqpyunsn", "m_$<};a\"hs-*", "7m7tjd7d2740", "u(k2:5{'$k{t", "elQvMQWcqGKU", "?j%HU{$,f:`r", "IsSP8ctkIHss", "X=G[q2s\"MXg1", ]);
}

#[test]
fn vectors_03() {
    t(3, "example.org", "user@example.org", "password", 2, 16, &["!<_}-]=)#\"~\"|&%/", "6503576030150097", "#?9=<!'88(9(?6<0", "EWMZSQPYUNSNBWXC", "M_$<};HWS-*(?#H@", "M7TJ7D2740B0B0FH", "U(K~2:{${T)+I1!A", "ewmzsqpyunsnbwxc", "m_$<};hws-*(?#h@", "m7tj7d2740b0b0fh", "u(k~2:{${t)+i1!a", "elQvMPQcGKUqyzrB", "?SU{$,f|:c`r~\\Ui", "IPcVtkI7Hqss2krZ", "=$Gs\"M8Xg1Igc;O$", ]);
}

#[test]
fn vectors_04() {
    t(4, "example.org", "user@example.org", "foobar", 1, 12, &["#~[={=<'/\\#>", "242832522353", "[(4^{:.158,5", "CORKECCEVAXM", "KK/%-}~'Z'^W", "04074Q3SZELA", "OS;{2942%G\\B", "corkeccevaxm", "kk/%-}~'z'^w", "04074q3szela", "os;{2942%g\\b", "VcursbxYvmkt", "[nY\"lVu:MJ*\"", "ml7yMngDM3eg", "46A}AH>n},Y;", ]);
}

#[test]
fn vectors_05() {
    t(5, "example.org", "user@example.org", "foobar", 1, 16, &["%#~[={=<'/#>\\(.#", "2428332523532551", "[(84^:.<18,5&9:{", "CORKECCEVYAMXQNJ", "K/%Y-}~'Z'WEQ)#I", "04G7343SZELAEHHJ", "KS;{298=2G\\B8Z?V", "corkeccevyamxqnj", "k/%y-}~'z'weq)#i", "04g7343szelaehhj", "ks;{298=2g\\b8z?v", "cuarbDxYvmktSXdU", "[Ylu[:MJ*sJ\"hd&L", "ml7ysMnDQe4g4VsV", "6A0QH>#},Y;x7bd?", ]);
}

#[test]
fn vectors_06() {
    t(6, "example.org", "user@example.org", "foobar", 2, 12, &["^\"]*~>$=!<$>", "334721379456", "`+:3@*'2*#}`", "NCWYYDZRZJQG", ".RUN]-`#@MV~", "ENHP5VAUBPUW", "3!M49J<`L)&F", "ncwyydzrzjqg", ".run]-`#@mv~", "enhp5vaubpuw", "3!m49j<`l)&f", "nBFjqszIgMIj", "oP_L@'AP>e<k", "T2YKDGqYO4eX", "p{XY~>JG0kG4", ]);
}

#[test]
fn vectors_07() {
    t(7, "example.org", "user@example.org", "foobar", 2, 16, &["^\"]*~>$\\=<$>![\\!", "3347137894562772", "`+:3@-*'3*}`+|^[", "NCWYYDZRZJGQKQRH", "[.RUN]-#@M~SPK%(", "NHP6LVAUBPUW03AH", "396JU<`L)&F;%:E-", "ncwyydzrzjgqkqrh", "[.run]-#@m~spk%(", "nhp6lvaubpuw03ah", "396ju<`l)&f;%:e-", "nBFjqzgxMIjsCRvF", "L@'AP>e<kC%{uBiK", "T2YKDGYeX6aB53FH", "fXY~>(GkGY4\":8N@", ]);
}

#[test]
fn vectors_08() {
    t(8, "example.org", "user", "password", 1, 12, &["^|$[}{<\"%~'}", "148283777289", "`{05.4&!))~;", "JGFCGTCETSAS", "+J]\\~HFYX_'F", "NDJQRMW6Q900", "FZF~XL;<S1+R", "jgfcgtcetsas", "+j]\\~hfyx_'f", "ndjqrmw6q900", "fzf~xl;<s1+r", "jqBXkvwgfZcW", "L*@aQAZIn?~k", "9hKWhyqI1piT", "zUk)y%311~$4", ]);
}

#[test]
fn vectors_09() {
    t(9, "example.org", "user", "password", 1, 16, &["^|$[/}{<\"%'}~[%[", "1348283777892888", "`{0~56.&))~;%|}-", "JGFGGTCETSASCRVU", "J]\\T~HFY_'F$\\X:&", "NDQRMW6Q90VTC75T", "4FZFL<;<OS+RJ&20", "jgfggtcetsascrvu", "j]\\t~hfy_'f$\\x:&", "ndqrmw6q90vtc75t", "4fzfl<;<os+rj&20", "hjqBXkvwfIcWGZtj", "L*@AZKxIn~,kqCBk", "9Why6fqIFpiTdaNl", "zyz%11~$4U{%m6<*", ]);
}

#[test]
fn vectors_10() {
    t(10, "example.org", "user", "password", 2, 12, &["\"\"{'+<+`_//*", "750733113705", "1{*38-7#;@)+", "HJEEIIYXLFDL", "PN+QT,$}AP%W", "NV8ILR675V87", "9W*@F&#|3ENS", "hjeeiiyxlfdl", "pn+qt,$}ap%w", "nv8ilr675v87", "9w*@f&#|3ens", "HleBaUbDrjPV", "bOK&x,}pQJ.v", "9OhzMm4q4GRA", "(61/[2bWx@f.", ]);
}

#[test]
fn vectors_11() {
    t(11, "example.org", "user", "password", 2, 16, &["\"\"{{'++`_//*<}#(", "7507311370653441", "41>{*3-7#;@+}=0\"", "PHJEEIIYXLDLFMOO", "PN+CQT,$}AWXQ-|Z", "N7V8ILR67879SFL9", "*@F3#|3$ENSPMGH%", "phjeeiiyxldlfmoo", "pn+cqt,$}awxq-|z", "n7v8ilr67879sfl9", "*@f3#|3$enspmgh%", "HreBaYUbrjPVLOPS", "bOK,}xPp)Q.vXX$+", "9OhzMq0Y4RAjMXls", "(6/[62btfD.Fg_tg", ]);
}

#[test]
fn vectors_12() {
    t(12, "example.org", "user", "foobar", 1, 12, &["\\~[/-?~`|@\\]", "976892335155", ",>#'\\5\"60|\\7", "LZIDSQNVCHEE", "DULK~W'AJI&H", "P8WZ7HM21T9D", "4H_6F9\"}V;:!", "lzidsqnvchee", "dulk~w'aji&h", "p8wz7hm21t9d", "4h_6f9\"}v;:!", "lRNvzexXgBgb", ",o/{r\"?ObPC!", "5ZqbyEx1yOWB", "\"uz#7;-zJ.Ag", ]);
}

#[test]
fn vectors_13() {
    t(13, "example.org", "user", "foobar", 1, 16, &["\\~[/-?`|@\\!]~?@;", "9376892335551311", ",>#'0\\5\"0|7=?'0-", "LIDSQNVCHEEZUYBJ", "DULK~WAII&HN-{@E", "P8WHM21T9D96GYZQ", "H69\"8}V;:!<H)Q}R", "lidsqnvcheezuybj", "dulk~waii&hn-{@e", "p8whm21t9d96gyzq", "h69\"8}v;:!<h)q}r", "lRNavzeYxBgbGygy", ",o/{}?ObP!TOcZOK", "ZqjbEx1y4WBLWI5u", "\"<u#-tz.AgHKj8=u", ]);
}

#[test]
fn vectors_14() {
    t(14, "example.org", "user", "foobar", 2, 12, &["|^-,:^=,'%&.", "719777647155", "33*4%%/7\\/#!", "DBQQAXCJXMOU", "G|@YQ#OW\"R$+", "VKS2Y44IJSOS", "@[.%0@>6!1D\"", "dbqqaxcjxmou", "g|@yq#ow\"r$+", "vks2y44ijsos", "@[.%0@>6!1d\"", "DaEvTyKqsQKl", "T-zmuUPw'@.@", "HumBPxV0C8xa", "<jQmlvTCYl6*", ]);
}

#[test]
fn vectors_15() {
    t(15, "example.org", "user", "foobar", 2, 16, &["|^]-,:^='%&.,-/;", "7197776647155065", "33*|%/78\\/#!!3>{", "DBQQAXXCJMOUXZVN", "|@JYQ#OW\"R!+]$C<", "VK2Y4I9JSOST029K", "@.%0@=>6Z!\"&7\\O$", "dbqqaxxcjmouxzvn", "|@jyq#ow\"r!+]$c<", "vk2y4i9jsost029k", "@.%0@=>6z!\"&7\\o$", "DaErvyKqQKlPSWnF", "T}-zQmUw'@@jY(}X", "HmEBP8xV0axakFHM", "jQmvC3?YQl*m:+v|", ]);
}

#[test]
fn vectors_16() {
    t(16, "example.net", "user@example.org", "password", 1, 12, &["_*%~~\")+!|[.", "288542537451", "{}@\"@_!':6\\\"", "IHPCHVKMACUA", "DO:Q`\"_\"\"O;R", "02MAHUPEHHVB", "R}1/`(;85C%^", "ihpchvkmacua", "do:q`\"_\"\"o;r", "02mahupehhvb", "r}1/`(;85c%^", "IQDDEqZIudrm", "MPsV(MmkRu)_", "YrsRUQL82P70", "62H/,<n^#igb", ]);
}

#[test]
fn vectors_17() {
    t(17, "example.net", "user@example.org", "password", 1, 16, &["_*%~>~\")+|[.!;\\%", "2885425345217545", "8{}@\"@_-!:\\\"=.%1", "IHPCVQKMACUAHNJA", "OQ`\"_\"\"O;R.<O\\^<", "0MHL0UPEHHVB6ZA3", "}/K(;8.5C%^8`KXZ", "ihpcvqkmacuahnja", "oq`\"_\"\"o;r.<o\\^<", "0mhl0upehhvb6za3", "}/k(;8.5c%^8`kxz", "IQDuDEqIdrRmUZiy", "bMPs^MmRNu)_Kn-&", "YsRQL2NP708NTPiU", "W32/,;<m^igbbpS>", ]);
}

#[test]
fn vectors_18() {
    t(18, "example.net", "user@example.org", "password", 2, 12, &["#]$~?~,..;!\"", "430482662941", ";//*),'>5^7`", "UDRXZZEYYRAY", "'@X(>QS$@)LY", "0W6J3JQUE2Q3", "8AYA0ON;?1^(", "udrxzzeyyray", "'@x(>qs$@)ly", "0w6j3jque2q3", "8aya0on;?1^(", "UvzgNVjBVpcX", "A#`\\pqu^RB%,", "mlX0V0sJCoN8", "F4,FqU76!v~q", ]);
}

#[test]
fn vectors_19() {
    t(19, "example.net", "user@example.org", "password", 2, 16, &["#]$~~,..;!\"#?-<{", "4304862669412209", ";//*)'>^57`7$8`3", "URXZZTEYYRAYDGVW", "'X(>Q@S$@)YYJ%U(", "0WJ3SJUE62Q368Y2", "8Y5A^ON;?1(P{|C3", "urxzzteyyraydgvw", "'x(>q@s$@)yyj%u(", "0wj3sjue62q368y2", "8y5a^on;?1(p{|c3", "CUvzgqVjBVcXPgqo", "qA#`\\qu^R,^~Ct%C", "mDX0VlsJCo8r48J4", "4F1UI6!r!v~q4JH}", ]);
}

#[test]
fn vectors_20() {
    t(20, "example.net", "user@example.org", "foobar", 1, 12, &["^!\"[)/=[[~^<", "443586298624", "1/}/$7@'8)>0", "SNTSEREYIUEY", "A>!.EAKA[RA}", "MFVWQ61B6PWP", "6{?SP[A4H]O<", "sntsereyiuey", "a>!.eaka[ra}", "mfvwq61b6pwp", "6{?sp[a4h]o<", "SRTrPGTgRajf", "y+?`<scgHvAm", "iouLam0X0MtF", "O-$m&OR)1tzP", ]);
}

#[test]
fn vectors_21() {
    t(21, "example.net", "user@example.org", "foobar", 1, 16, &["!\"[)/=[[-~^<^[_;", "4443586298642575", "/}/$7@'80>0&<<;[", "SNTSEEYIUEYRLZLU", "A\"!.EKA[RA}L;SE]", "4MFWQ6BN6PWPHE8U", "6{?SP[AH<S=3Y%AD", "sntseeyiueyrlzlu", "a\"!.eka[ra}l;se]", "4mfwq6bn6pwphe8u", "6{?sp[ah<s=3y%ad", "STrmPGETgRjfAILk", "y+`]<sgHvgmMQ\\jW", "iSo9gLamX0tFEdzK", "!O-4$&OtxzPO&QyT", ]);
}

#[test]
fn vectors_22() {
    t(22, "example.net", "user@example.org", "foobar", 2, 12, &["|>.}*|?+(~%}", "909885389253", "\"1![:!+39$)}", "TBTTOAJYDBTB", "G(IHE~/&K`M:", "R9QONU0PWL27", "{<KWO5D[_4,|", "tbttoajydbtb", "g(ihe~/&k`m:", "r9qonu0pwl27", "{<kwo5d[_4,|", "TcWFriVOnPsL", "\"VQSJ#'MxLu@", "R0dNatiFQz7L", "}dHv2NUJJRg[", ]);
}

#[test]
fn vectors_23() {
    t(23, "example.net", "user@example.org", "foobar", 2, 16, &["|>.}*|?+(~}%%(]`", "9099853892538860", "\"1![!=+3$)}>>9#0", "TTTOAJYTDBTBBLZX", "(I+HE~&KG`M:O}|&", "MR9QONPWL277UZ9Q", "<<XKOD[_4,|21>F%", "tttoajytdbtbblzx", "(i+he~&kg`m:o}|&", "mr9qonpwl277uz9q", "<<xkod[_4,|21>f%", "TWHariVOnPsLCPBU", "\"QSJ'MLu@Hr<XO\\=", "dNTatiFQ7LtS1suS", "dHlvN$U5JJ[4EF|W", ]);
}

#[test]
fn vectors_24() {
    t(24, "example.net", "user", "password", 1, 12, &["**:=[>.$}|<_", "330500449036", "`6{{'),7-435", "FILFOHPMUUOE", "(-`P}>})S-V.", "BK6H4WGP2WZ0", "2<7:Y\"Q&@<E#", "filfohpmuuoe", "(-`p}>})s-v.", "bk6h4wgp2wz0", "2<7:y\"q&@<e#", "lfrJaWsPVWDJ", "QLT?-:qd~kcj", "N1xiGAfaz8oA", "3n|u}%1S+E!m", ]);
}

#[test]
fn vectors_25() {
    t(25, "example.net", "user", "password", 1, 16, &["**:[>.$}|<_==}}|", "3305000490364673", "`6{{')-435,1/'06", "FIILFOHPUUOEMUSG", "(-`ZP}>}S-.*^L&T", "BK6HWGP22Z08D1LT", "<7:Y\"Q&@#U/K6.A4", "fiilfohpuuoemusg", "(-`zp}>}s-.*^l&t", "bk6hwgp22z08d1lt", "<7:y\"q&@#u/k6.a4", "fryJaAWsVWDJLhKJ", "LT?:qd~cj+<q()Aw", "Nxi3Gfa8oAXGhWaE", "3n|}%HlS8!mG#yk<", ]);
}

#[test]
fn vectors_26() {
    t(26, "example.net", "user", "password", 2, 12, &["-|]}$%/=(-)'", "866664383666", "2*|\"=?}86&7\"", "YCCDOALVICJH", "I[A/\\|E-W>$'", "QIJO8D17VAJ4", "A(Y_)0__{')H", "yccdoalvicjh", "i[a/\\|e-w>$'", "qijo8d17vaj4", "a(y_)0__{')h", "ybUgESkppvgx", "S?cJs\\/Q[MIe", "0GmPZLiNCV9P", "Ab?yM0!nXeUN", ]);
}

#[test]
fn vectors_27() {
    t(27, "example.net", "user", "password", 2, 16, &["-|}~$%/=(-)']?>|", "8666643866637499", "2*\"=?}8&77@\"?(#/", "CYCCDOALVCJHIKXP", "A[A\\,|E-W>$'#GHU", "I0JO8DK1VAJ4GYC5", "A()N__{1')H%}-F}", "cyccdoalvcjhikxp", "a[a\\,|e-w>$'#ghu", "i0jo8dk1vaj4gyc5", "a()n__{1')h%}-f}", "ybUgQyEkpvgxpWcc", "YS?pcJ\\/QMe^}AwL", "w0GTmPZLNV1P0GPt", "yMc!TnX?eUN1O6\\t", ]);
}

#[test]
fn vectors_28() {
    t(28, "example.net", "user", "foobar", 1, 12, &["!`%%{]\"$.]=_", "808705742253", "4\"%+2~`32)+8", "ONNEIHIJDSET", "#-CYUF~R\"|?.", "QAVEUDVY4SIJ", "60QSU1!T_|H}", "onneihijdset", "#-cyuf~r\"|?.", "qaveudvy4sij", "60qsu1!t_|h}", "UOgOkxmoGMEv", "ehfbLL%Fk(gv", "lo9X4xARHKQv", ".{Jr3.usRde@", ]);
}

#[test]
fn vectors_29() {
    t(29, "example.net", "user", "foobar", 1, 16, &["!`%%{]$~.]=_\"`{;", "8087057542235016", "4$%+2~`3)+82{3%?", "ONEIHIJTDSETNHLT", "#-~CYU~R\"|.VG'!;", "QAVEFUD1VYIJ0WSP", "6QUX1;T_|H}=0`G3", "oneihijtdsetnhlt", "#-~cyu~r\"|.vg'!;", "qavefud1vyij0wsp", "6qux1;t_|h}=0`g3", "OLgOkxmGMEkvokRJ", "<ehYfLFk(vgvBvNl", "o9XxAhHKQvZRU03X", "{J9rl.?sde@+|DY`", ]);
}

#[test]
fn vectors_30() {
    t(30, "example.net", "user", "foobar", 2, 12, &["}';}%,>(/&\\>", "011103641636", "62.5=;#}_2).", "EXBQEBROEQZF", "IP?,$G^?E.&C", "SHVSRC7DA07S", "=%K]7T1G#<~3", "exbqebroeqzf", "ip?,$g^?e.&c", "shvsrc7da07s", "=%k]7t1g#<~3", "EkyAvVlaBYZZ", "gw'aQho<S;!A", "mPJACrTET3xg", "%UuOzw/>MA7/", ]);
}

#[test]
fn vectors_31() {
    t(31, "example.net", "user", "foobar", 2, 16, &["}';;}%,>/&\\>(%&$", "0110364163261280", "672|.5=;#_)./:*0", "EXPBQEROEQZFBQKH", "I,$YG^??E.&CPO(*", "SHVSR75NDA0SYTTT", "=K]Q7G#<~3?3)7T>", "expbqeroeqzfbqkh", "i,$yg^??e.&cpo(*", "shvsr75nda0syttt", "=k]q7g#<~3?3)7t>", "EyAvVZlaYiZZKnpM", "gw'ahr<TS;Ao??)$", "mJACLTETxg7R0uiP", "J%UO+zbw/M/oh7C3", ]);
}
