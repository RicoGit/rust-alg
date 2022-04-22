//! 567. Permutation in String

use std::collections::HashMap;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        // it is safe to convert to bytes, because s1 and s2 consist of lowercase English letters
        let s1: Vec<u8> = s1.bytes().collect();
        let s2: Vec<u8> = s2.bytes().collect();

        // count frequencies
        let mut s1_freq = Solution::frequencies(&s1);

        if s1.len() > s2.len() {
            return false;
        }

        let window_size = s1.len();
        let mut window = Solution::frequencies(&s2[0..window_size]);

        let mut start_idx = 1;
        loop {
            if window == s1_freq {
                return true;
            }

            let end_idx = start_idx + window_size - 1;
            // println!("{:?}, {}-{}", window, start_idx, end_idx);

            if end_idx > s2.len() - 1 {
                return false;
            }

            // remove elem from start, add from end
            let old_elem = s2[start_idx - 1];
            match window.remove(&old_elem) {
                None => (),
                Some(1) => (),
                Some(prev) => {
                    window.insert(old_elem, prev - 1);
                }
            };
            let new_elem = s2[end_idx];
            match window.remove(&new_elem) {
                None => window.insert(new_elem, 1),
                Some(prev) => window.insert(new_elem, prev + 1),
            };

            start_idx += 1;
        }
    }

    fn frequencies(arr: &[u8]) -> HashMap<u8, usize> {
        let mut freq = HashMap::<u8, usize>::new();
        for ch in arr {
            match freq.remove(&ch) {
                None => freq.insert(*ch, 1),
                Some(prev) => freq.insert(*ch, prev + 1),
            };
        }
        freq
    }
}

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::check_inclusion("ab".to_string(), "eidbaooo".to_string()),
            true
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::check_inclusion("adc".to_string(), "dcda".to_string()),
            true
        );
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::check_inclusion(
            "ajsbmzgzwrjwvzfxjqasaoreuulzmypibgubyxdbbinkdmmibetcnsekvprichjgjviodllxyqcrbzgwrkmrrtnmfljjtzajghpoyxcqvhouufwfjtclfrrecqqydzuyoawvnwcvpquebdqzssctviwyszhjpzyrhjhczhvfwvvmqffsmdxljkuzxgumsgeaxpogcgstgmqgmaxvtvtpvkipavimzthncsmdhzbylrgzkluhncojblhrdgjwynclbzbloqtupkltrokhhfdbxdncfmaeujfvtwcnqnrfrxukjeprefjttcptdpgjjanipbqtqorfwugztbjszzwekqmllmnupsanxqgrbzdelmqxjwmdlkbvcpxjyasvldwjftetktovoxkbksrjakrsvggwdrgtvpqfgeibybhkdkeckuokduykynvmreyirowvkxxnxjzwtxtasgaaaheefskmyuzyarfyycgiznvlowlaonpacjkwlhnwekhbziitcidcslknvpzplzcajixdhuzrdvhprjyfknepwjbxtgfhduurvixibfswzousjjnydblheoysyxptcpsxgiojcvpegnhbdbnyudslytzsytukrhcghwbpshmqmftpnqwitumzovcnutbcvghvvgasfoerywwejpndtkevysmygwcmlhamglmxiwvysseyphozwnytjecunsbesywipayxxvtypvdevlimywvpdvekzkqigkpgjychxemqlwocnivcyysmvwnqxblubdwynkegwmjzsdclwxyvlpjcoqqkbgtejezffucifrxaehmfcoieyscgiotjzwxpdtnjpinzlrybmdvlndppbxxnlnsdblujiuutzsgcqolbofvntqygsqozwjvnlgvsqphtsujcqsiqwlkdazdypivuriagqedzvxexkvkzlnnyydoavijoubqkpqpwcahtxfuvnkvltbdhtoehxzhwpxswmulwmjtucwlgwlrwpaoedjgraryzcvblaieqfinacqlpxsnezmofdjhvhmimysatnmnmzdravskbkswqobfxclzikeluahnulrsouiltqguoyjcpeioyrtnhcbyncwiyonzhsjwcnevmizkhcrhzgcbsukjsllnvlnqesqnifvkxyfxxanysjoujdsrhgwfarcbvqacedqdabqgguqtjtxztasvlyxjqajfsevrqyaxmezbqtxlfgrdltlskzszgcskhxndnvbrmrtclurrogsvfmeyavffpdmyquyhmyrgoprqtthqmutbomdwqmclpuuybyytizgobsqgplulesxrwokzcgojcskbtlbljtaqkmjkilvhpsyvlioirhcdsqtsvsddhqpgbpziemtvtafdcshoxqagrimqfehnjgnemgglugsixdfebvkekcvsmiwcwuysohwhwisvpkkrbwaqucmlbbulvvqwvpsurexghsnkxoxybhrrulvoenddlobqojsvdthcsrolgfqfamofeezmcqhgrlrwkobpqzvhkyypsaoyvaukxlmatnsygzbdnrvjhchrqaptanlqrunrnywxccdslymkstxpesrioqnuiamboiptmsqjgizkhdalkkcllfqxqsmvwuchrfmqynraxspqsvofqwfedfbxupyuostevhcrjypyffvbmjbfhrtzwwmqffiigwelokmgleogyxjrgbfrutaabvmevlsumwyeqffajfdvewwslehtdypputixbltwokylcsdtznvvhadzztkidhlufkizwkfaywybckwauovumplymvrsytbwnmjcvaheoowyhitihzddkzxqjiyvhhqqtxykvgztgcagcitgunlnefwgycojtijuuhygjfgxvyr".to_string(),
            "prtwaywvvyqetvwyakpvekjhirlrtxxzyaodvwigtrdaeiizykaebcjuoenyhaoadnvxaasbecfpovauvvffvcnchdgffbbdhsrxigmtljvoqdiwjvsbpbkppvbxkxphzdkqumpuoacghmmicfjmondzxppomayqnvkclqzbuptslgzjnhjqggwedeuxormigkwjkqsgebufhhfdhqfuifnyboecqvnjhcgztlwoahcqwrksdcedhmqiqxjbqhaacrosnupiesatqfeglxfqgpsfiotytnrvgcvntmxblkkdblatbesaransxcvzeconimbuweevufhduxzcvozgrmkvxdwsiuheqqjjxjimliwehsaxmwjrncapcadzswwglxfnxsnpdbbyvxdwrrflwpjduxnwrjdogyrejrkmjpdncvxwqfcfpfcszzkislhxergkssurbjnuxzpzgkbegtkzbpdutwipdygmhbvspuwxxsapddzhppkkfxbrylvipexzubrzfbmhcqutjnqjbenmqfnowvujicgawxxynnzfubskzyihuohhahflqkrfkvvaimaptzqybjgpeomviozqnbmlgwftgeoztgzacujrolhlwkhjnmklhfyclxzmhfanwgvfhlexuxlpusgbgfdvtlccykeflwinskcahspqlesxrybqegqjmpzeiyhgvombebfsuqvnwqcifznxauupzqxmtejumabbwpjjnqvqrekwaackfvvcvbdcfidqytjhmswyovravwdaahiftspjdxwhbmiabtehdkzagabqiokpnmtpkhqfwacexshckfplcruinijigqdbsjqnskocnhchfsjytqkpbrodtffbsuislmyqpuvxruetrcobbibvipsnkrkrbvhguglpjbbotyrpletcpvfsptvlcdklkfrzjmyzzszzevgkezwltnykylohxoivhmfcmmzcxtmnjcbkrvxndxoisymvfengtcezyxozcyfoyvhqdnrxtfubmvvjxkzatdwcgkyplnzduxbvksxdlnstspxummjqukbcswezkkuwvepbjqoyewnbhiwmtxriunlkwdtgwgmgoqckfjcyxkoefbaizjkavybjlhqgfubhkeberznmokjwwbpkndbirtuvyuqqvfevpxedctnkgryructmtzntfkvpxdqznhtnipfeajehxsyraosvvlaivlehljrynbeczxcigrklnbfnlnqulpwoyvpokmwfjzdwwzpwmgiygxtnelwisdcvoflanozvlsadxngdjjbhyniprfwidzhsbtozgolyjerfzmsyeimyjdvzwcauenjnbymdvfoqepgcpcybqxvumijejcfettsdfsvdcgbylwsuaaemkxrvtilmrkscphywzlyrwahhmomqjhelgdgthiaekrbvmsarndqgodtrnowdjnspjjphomabvdbujimrezxfkhzxlkgagibgcavxemdkwtjltnufdfnkvtnhfsoqpntiwtnghlfhpupwhrdcbgjogbedsdyrtzlfcupbkubgzibcgoeuqehnhcfsztbfprrgmfphchqcgjjfxxggrvbpqawjwumuxkbiuujrzpgvwndckqhuhwpwljdntaqyrtwwkydbbduuqznqmbvssqrzixdgqswurstvpvboxykpndvmcrhccfduhjwbczqchwosugacufnoiramiygadmnbkyhtdcmlrliortwobmsirnbmqjchjrfesuodvaxagwfuhqqebjxqalnxaseyrbkmrgajtjmqeimyhrhrrrwvzwhhlddwdgqjvhtxrgntnyxvhjngksndttfqprcwjkoiztsuysbjvjpralkkgmdfzlfiayrbjezoyphgkictjpiqfrmdizcwcjcpyhrhmovytnuawtzdlclnvfkcoksvigpmvnptnqausmbvquhuhthkstmtmugjpjxgslluuxxfhyqtmedsmwglxuuzurkgmqdytrcvnvxrlqssjwoqlvmkmyeftktkmfulcutalzudpsmqvgjtfaegcszttcgxoosgqjhiutpnfmfikekfvokvjzxgjfbefrptsjmqijudwkajsdmztzwrjwvzfxjqasaoreuulzmypibgubyvdbbinkvmmibetcnsekvprichjgjviodllxyqcrbzgwrkmrrtnmfljjtzajghpoyxcqvhouufwfjtclfrrecqqydzuyoawvnwcvpquebdqzssctviwyszhjpzyrhjhczhvfwvvmqffsmwxljkukxgumsgeaxpogcgskgmqgmaxvtvtpvkipavimzthncsmdhzbylrrzkluhncojblhrogjwynclbzbloqtupkltrokhhfdbxdncfmaeujfvtwcnqnrfrxukpeprkfjttcptdpgjjaniybqtqorfwugztbjszzwekqmllmnupsanxqgrbzdelmqxewmdlkbvcpxjeasvldwsftetktovoxkbksrjaersvggwdrgtvpqfueibybhkdzeckuokduykynvmreyirowvklxnxjzwtxthsgaaaheefskhyuzyarfyycgiznvlowlaonpacjtwlhnwekhbziitcidcslknvpzplzcajixdhuzrdvhprjyfknepwjbxtgfhduurvixibfswzousjjnydblheoysyxptcpsxgiojcvpegembdbnyudrlytzsytukrhcghwbpshmqmftpnqwirukzovcnutbcvgvvvgasfoerywwejpndtkovysmygwcmfhamglmxiwvysseyqhozwnytjecunsbesywipayxxvtypvdevlimywvpddjkzkqigkpgjychxemplwocnivcyysmvwnqxblubdwynkegwmjzsdclwxyvlpjcoqqkbgtejezffucifrxaehmfcoieyscgiotjzwxpdtnjpinzlryjmdvlndppbxxnlnsdblujiugtzsgcqolbofvntqygsqozwjvnlgvsqphtsujcqsiqwxkdazdypivuriagqedzvxexkvkzlnnyydoaxijoubqkpqpwcahtxfuvnkvltbdhtoehxzhwpxswmulwmjoucwlgwlrwpaoedjgrarypcvblaiyqfinacqlpxsnezmofdjhvhmimysatnmnmzdravskbkswqobfxclzikeluahnulrsouiltqguoyjcpeioprtnhcbyncwiyonzhsjwcnevmizkhcrhzgcbsukjsllnvlnqesqnifvkxyfxxanysboujdsrhgwfarcbvqacesqdabqgguqtjwxztasvlyxjqajfsevrqyaxmezbqtxlfgrdltlskzszgcskhxndnvbrmrtclutrogsvfmeyavffjdmyquyhmyrgdprqtthqmutbomdwqmclxuuybgytizgobsqgplulesxrwokzcyojcsmbtlbljvaqkmjkilvhpsyvlioirhcdsqtsvdddhqpgbpziemtvtafdcshoxqagrimqfehnjgnemggljgjixdfnbhkekcvsmiwcwuysohwhwisvzkkrbwaqucmlbbulvvqwvpsurexghsnkxoxyhhrrulvoenddlobqojsvdthcsgolgfqfamofeezmcqhgrlrtkobpqzvhkyypsaoyvaukxlmatnsygzbdnrvjhcbrqaptanlqrunrnywxccdslymkstppesrioqnuiamboiptmsqjgizkhdalkkclllqxqsmvwuchrfmqynraxspqsvtfqwfedfbxupyuostevhcrjypyffvbmubfastzwwmqffiigwelowmglgogyxjrgbfrutaabvmevlsumwyeqffajfdvewwslehtbypputixbltdokylcsdgznvvhadzztkidhlufkizwkfaywybckkauovumplymvrsytbwnmjcvaheoewyhitihzddkzxqjiythhqqtxykvgztecagcitgunlnefwgycojtijuuhygjfgxvyrkukwwldvsbhvehtobdgmodtbmaiuccendlwyzcbojtnbzchlkubdvvoyjijzkgyscuelctrpitpfaftxufwxikoqcrsxlzoskvxncumcbhigxobqgpwhbdqxtjamzxuelrvlnijthgoljmrarcgosqlqzcxtukxjuxynnqkywiptwwtchkwndqfhhzsxeavryaybjcuidjbukzmcrwgahpqrmsgztknkfoaeypaemyorbqejfgifraujzjdzkhtdzhadhslqhwgsmuzmmqshsqjwdzssswribmeknttxvodehjosybfleqrgofwdieirdgtftipnqqmdfrgjttkhicdugvtaqxtibveuszsicjpglmdqxkieznybwkymliycilxssaoaffjahuidldmuvqvbftgqxkfwzvkuatgqbjezgfviubuubrukccymrwjgzfftytktbihgqdbgawdsqufxwvbcgssihzqkmrgyrbtdykrihhxtsbsygjyzmyjyycgvtpndoonqdhgrhdnjerjhmbiwhzrzwkduscdtjcxndjpvldzptewgwvkkqakknraduuzvssgxmpvcrhnplwsubqplminqelizpenrcnopyoogdhgmcztulqcmcrglhxsyavtfwavhauscsrtuvyzibpthznsqngwpxjdcgdrisynpjfxliirwyskfjyicfufxjlqrawktujyfttjzlkgytmmkurrwsuvizisewsgaqptntofjqodiliduitavocubvnskliuffpsvftknzlapmgjqffmbsrvduhhigjnskkhbcqequerdzqgcyjlqoxscbxgjixkogjzpvzrduniovkkvatvdgzpupkyjavxmhokgouuitoftdhhrzrkmxtbogxileadtrvbndmxggxegqfacruvqqlhxjmgurwbggqfbkccyqsnkpvhfsgjnicifphizihllesdrqkasdhdkwjzsvsvmwozxoyelirilnqywggxfrwcufbwasgsooofkjwcqgigobkojrarkdqdjclllceosjwuyrwvchusbuyablmjjwmgxpatxkgvmpjhzuyswcfkmgnzforanvkhklhwrlfnpgxsnanskwtptcdcqkiprmzortjrulgzmkyetpufnbpprkjerorforfnbrhlwsinqxehufgjvlglmeqkkfrnxppofdjpvgxxgwjdsxhdfbwscfkgtxxdxnsjsrtiyylmatqvglcidbofstcilzpgodcephqudwqkjyqjfouvdrslrwahbamkavzokwovemypzsqzksusrgvdnezghbyavxbrfqmbrabzcfurnnkvhljdufwbtjbkurgnhikketdzqbeoiulavhymvnfqybocoxwfkoxijouqlkiriozrmoipoxfkirgtscjyqeuubzbdaequjchbrthgkwpwexlzritpbghwbtkdagvummxtctmhbxxfibovfytrcgqggsfthyrhzejyukhyycekgdcytnhloyzmxneqnywmgfdntojwkzdmdlxnkkftafpbqveskwxdixeegukgzrjsxwbneoimkrngxigycjwzghrigfapoklmbmttgqjvyffkyxscjelcjvofforoqqsqvrumntgxwgwytlfleamuatuyxitrhvjjiacaoezmorhvlxmrebfxhcfopjrkreiyxzixzwutarxrqyhtnvuiaczscaguldraqslvvqutzugthofxfadbiygmjszftwywdgwrdalmhneserexzshikthojoqgjkwobgyfrumaxoxaqobzlvdzsoxzucvduimqxbyqmvxvtbwgalhxndrhstprjyywnogtoawcroiztdkpndwvkztxbrzfriandnaaqiqoopfzdsretxbvjqboppxkbgwmahyicwpwllvrlyymezomwkwtogqgbqcvexkleumfgfnhikojuhfszvvnswcegpshnpfnvfvpjcxsjtaitkylsoijdzadzrrevkrtevdzcybvjdqpkeeobbnrzbjlyrhaxzgtmbjzbjpsplghdlbwbyqefbyzindtzgsksufnipj".to_string()
        ), true);
    }
}
