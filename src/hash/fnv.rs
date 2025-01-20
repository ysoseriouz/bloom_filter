// ref: https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function
// ref: http://www.isthe.com/chongo/tech/comp/fnv/index.html#FNV-1a

const FNV_OFFSET_BASIS: u32 = 0x811c9dc5;
const FNV_PRIME: u32 = 0x01000193;

// 32-bit FNV-1a
pub fn fnv(data: &[u8]) -> u32 {
    let mut hash = FNV_OFFSET_BASIS;
    for &byte in data {
        hash ^= byte as u32;
        hash = hash.wrapping_mul(FNV_PRIME);
    }

    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repeat_10(bytes: &[u8]) -> Vec<u8> {
        (0..10).flat_map(|_| bytes.iter().copied()).collect()
    }

    fn repeat_500(bytes: &[u8]) -> Vec<u8> {
        (0..500).flat_map(|_| bytes.iter().copied()).collect()
    }

    #[test]
    fn test_fnv() {
        assert_eq!(fnv(b""), 0x811c9dc5);
        assert_eq!(fnv(b"a"), 0xe40c292c);
        assert_eq!(fnv(b"b"), 0xe70c2de5);
        assert_eq!(fnv(b"c"), 0xe60c2c52);
        assert_eq!(fnv(b"d"), 0xe10c2473);
        assert_eq!(fnv(b"e"), 0xe00c22e0);
        assert_eq!(fnv(b"f"), 0xe30c2799);
        assert_eq!(fnv(b"fo"), 0x6222e842);
        assert_eq!(fnv(b"foo"), 0xa9f37ed7);
        assert_eq!(fnv(b"foob"), 0x3f5076ef);
        assert_eq!(fnv(b"fooba"), 0x39aaa18a);
        assert_eq!(fnv(b"foobar"), 0xbf9cf968);
        assert_eq!(fnv(b"\0"), 0x050c5d1f);
        assert_eq!(fnv(b"a\0"), 0x2b24d044);
        assert_eq!(fnv(b"b\0"), 0x9d2c3f7f);
        assert_eq!(fnv(b"c\0"), 0x7729c516);
        assert_eq!(fnv(b"d\0"), 0xb91d6109);
        assert_eq!(fnv(b"e\0"), 0x931ae6a0);
        assert_eq!(fnv(b"f\0"), 0x052255db);
        assert_eq!(fnv(b"fo\0"), 0xbef39fe6);
        assert_eq!(fnv(b"foo\0"), 0x6150ac75);
        assert_eq!(fnv(b"foob\0"), 0x9aab3a3d);
        assert_eq!(fnv(b"fooba\0"), 0x519c4c3e);
        assert_eq!(fnv(b"foobar\0"), 0x0c1c9eb8);
        assert_eq!(fnv(b"ch"), 0x5f299f4e);
        assert_eq!(fnv(b"cho"), 0xef8580f3);
        assert_eq!(fnv(b"chon"), 0xac297727);
        assert_eq!(fnv(b"chong"), 0x4546b9c0);
        assert_eq!(fnv(b"chongo"), 0xbd564e7d);
        assert_eq!(fnv(b"chongo "), 0x6bdd5c67);
        assert_eq!(fnv(b"chongo w"), 0xdd77ed30);
        assert_eq!(fnv(b"chongo wa"), 0xf4ca9683);
        assert_eq!(fnv(b"chongo was"), 0x4aeb9bd0);
        assert_eq!(fnv(b"chongo was "), 0xe0e67ad0);
        assert_eq!(fnv(b"chongo was h"), 0xc2d32fa8);
        assert_eq!(fnv(b"chongo was he"), 0x7f743fb7);
        assert_eq!(fnv(b"chongo was her"), 0x6900631f);
        assert_eq!(fnv(b"chongo was here"), 0xc59c990e);
        assert_eq!(fnv(b"chongo was here!"), 0x448524fd);
        assert_eq!(fnv(b"chongo was here!\n"), 0xd49930d5);
        assert_eq!(fnv(b"ch\0"), 0x1c85c7ca);
        assert_eq!(fnv(b"cho\0"), 0x0229fe89);
        assert_eq!(fnv(b"chon\0"), 0x2c469265);
        assert_eq!(fnv(b"chong\0"), 0xce566940);
        assert_eq!(fnv(b"chongo\0"), 0x8bdd8ec7);
        assert_eq!(fnv(b"chongo \0"), 0x34787625);
        assert_eq!(fnv(b"chongo w\0"), 0xd3ca6290);
        assert_eq!(fnv(b"chongo wa\0"), 0xddeaf039);
        assert_eq!(fnv(b"chongo was\0"), 0xc0e64870);
        assert_eq!(fnv(b"chongo was \0"), 0xdad35570);
        assert_eq!(fnv(b"chongo was h\0"), 0x5a740578);
        assert_eq!(fnv(b"chongo was he\0"), 0x5b004d15);
        assert_eq!(fnv(b"chongo was her\0"), 0x6a9c09cd);
        assert_eq!(fnv(b"chongo was here\0"), 0x2384f10a);
        assert_eq!(fnv(b"chongo was here!\0"), 0xda993a47);
        assert_eq!(fnv(b"chongo was here!\n\0"), 0x8227df4f);
        assert_eq!(fnv(b"cu"), 0x4c298165);
        assert_eq!(fnv(b"cur"), 0xfc563735);
        assert_eq!(fnv(b"curd"), 0x8cb91483);
        assert_eq!(fnv(b"curds"), 0x775bf5d0);
        assert_eq!(fnv(b"curds "), 0xd5c428d0);
        assert_eq!(fnv(b"curds a"), 0x34cc0ea3);
        assert_eq!(fnv(b"curds an"), 0xea3b4cb7);
        assert_eq!(fnv(b"curds and"), 0x8e59f029);
        assert_eq!(fnv(b"curds and "), 0x2094de2b);
        assert_eq!(fnv(b"curds and w"), 0xa65a0ad4);
        assert_eq!(fnv(b"curds and wh"), 0x9bbee5f4);
        assert_eq!(fnv(b"curds and whe"), 0xbe836343);
        assert_eq!(fnv(b"curds and whey"), 0x22d5344e);
        assert_eq!(fnv(b"curds and whey\n"), 0x19a1470c);
        assert_eq!(fnv(b"cu\0"), 0x4a56b1ff);
        assert_eq!(fnv(b"cur\0"), 0x70b8e86f);
        assert_eq!(fnv(b"curd\0"), 0x0a5b4a39);
        assert_eq!(fnv(b"curds\0"), 0xb5c3f670);
        assert_eq!(fnv(b"curds \0"), 0x53cc3f70);
        assert_eq!(fnv(b"curds a\0"), 0xc03b0a99);
        assert_eq!(fnv(b"curds an\0"), 0x7259c415);
        assert_eq!(fnv(b"curds and\0"), 0x4095108b);
        assert_eq!(fnv(b"curds and \0"), 0x7559bdb1);
        assert_eq!(fnv(b"curds and w\0"), 0xb3bf0bbc);
        assert_eq!(fnv(b"curds and wh\0"), 0x2183ff1c);
        assert_eq!(fnv(b"curds and whe\0"), 0x2bd54279);
        assert_eq!(fnv(b"curds and whey\0"), 0x23a156ca);
        assert_eq!(fnv(b"curds and whey\n\0"), 0x64e2d7e4);
        assert_eq!(fnv(b"hi"), 0x683af69a);
        assert_eq!(fnv(b"hi\0"), 0xaed2346e);
        assert_eq!(fnv(b"hello"), 0x4f9f2cab);
        assert_eq!(fnv(b"hello\0"), 0x02935131);
        assert_eq!(fnv(b"\xff\x00\x00\x01"), 0xc48fb86d);
        assert_eq!(fnv(b"\x01\x00\x00\xff"), 0x2269f369);
        assert_eq!(fnv(b"\xff\x00\x00\x02"), 0xc18fb3b4);
        assert_eq!(fnv(b"\x02\x00\x00\xff"), 0x50ef1236);
        assert_eq!(fnv(b"\xff\x00\x00\x03"), 0xc28fb547);
        assert_eq!(fnv(b"\x03\x00\x00\xff"), 0x96c3bf47);
        assert_eq!(fnv(b"\xff\x00\x00\x04"), 0xbf8fb08e);
        assert_eq!(fnv(b"\x04\x00\x00\xff"), 0xf3e4d49c);
        assert_eq!(fnv(b"\x40\x51\x4e\x44"), 0x32179058);
        assert_eq!(fnv(b"\x44\x4e\x51\x40"), 0x280bfee6);
        assert_eq!(fnv(b"\x40\x51\x4e\x4a"), 0x30178d32);
        assert_eq!(fnv(b"\x4a\x4e\x51\x40"), 0x21addaf8);
        assert_eq!(fnv(b"\x40\x51\x4e\x54"), 0x4217a988);
        assert_eq!(fnv(b"\x54\x4e\x51\x40"), 0x772633d6);
        assert_eq!(fnv(b"127.0.0.1"), 0x08a3d11e);
        assert_eq!(fnv(b"127.0.0.1\0"), 0xb7e2323a);
        assert_eq!(fnv(b"127.0.0.2"), 0x07a3cf8b);
        assert_eq!(fnv(b"127.0.0.2\0"), 0x91dfb7d1);
        assert_eq!(fnv(b"127.0.0.3"), 0x06a3cdf8);
        assert_eq!(fnv(b"127.0.0.3\0"), 0x6bdd3d68);
        assert_eq!(fnv(b"64.81.78.68"), 0x1d5636a7);
        assert_eq!(fnv(b"64.81.78.68\0"), 0xd5b808e5);
        assert_eq!(fnv(b"64.81.78.74"), 0x1353e852);
        assert_eq!(fnv(b"64.81.78.74\0"), 0xbf16b916);
        assert_eq!(fnv(b"64.81.78.84"), 0xa55b89ed);
        assert_eq!(fnv(b"64.81.78.84\0"), 0x3c1a2017);
        assert_eq!(fnv(b"feedface"), 0x0588b13c);
        assert_eq!(fnv(b"feedface\0"), 0xf22f0174);
        assert_eq!(fnv(b"feedfacedaffdeed"), 0xe83641e1);
        assert_eq!(fnv(b"feedfacedaffdeed\0"), 0x6e69b533);
        assert_eq!(fnv(b"feedfacedeadbeef"), 0xf1760448);
        assert_eq!(fnv(b"feedfacedeadbeef\0"), 0x64c8bd58);
        assert_eq!(fnv(b"line 1\nline 2\nline 3"), 0x97b4ea23);
        assert_eq!(fnv(b"chongo <Landon Curt Noll> /\\../\\"), 0x9a4e92e6);
        assert_eq!(fnv(b"chongo <Landon Curt Noll> /\\../\\\0"), 0xcfb14012);
        assert_eq!(fnv(b"chongo (Landon Curt Noll) /\\../\\"), 0xf01b2511);
        assert_eq!(fnv(b"chongo (Landon Curt Noll) /\\../\\\0"), 0x0bbb59c3);
        assert_eq!(
            fnv(b"http://antwrp.gsfc.nasa.gov/apod/astropix.html"),
            0xce524afa
        );
        assert_eq!(
            fnv(b"http://en.wikipedia.org/wiki/Fowler_Noll_Vo_hash"),
            0xdd16ef45
        );
        assert_eq!(fnv(b"http://epod.usra.edu/"), 0x60648bb3);
        assert_eq!(fnv(b"http://exoplanet.eu/"), 0x7fa4bcfc);
        assert_eq!(fnv(b"http://hvo.wr.usgs.gov/cam3/"), 0x5053ae17);
        assert_eq!(fnv(b"http://hvo.wr.usgs.gov/cams/HMcam/"), 0xc9302890);
        assert_eq!(
            fnv(b"http://hvo.wr.usgs.gov/kilauea/update/deformation.html"),
            0x956ded32
        );
        assert_eq!(
            fnv(b"http://hvo.wr.usgs.gov/kilauea/update/images.html"),
            0x9136db84
        );
        assert_eq!(
            fnv(b"http://hvo.wr.usgs.gov/kilauea/update/maps.html"),
            0xdf9d3323
        );
        assert_eq!(
            fnv(b"http://hvo.wr.usgs.gov/volcanowatch/current_issue.html"),
            0x32bb6cd0
        );
        assert_eq!(fnv(b"http://neo.jpl.nasa.gov/risk/"), 0xc8f8385b);
        assert_eq!(fnv(b"http://norvig.com/21-days.html"), 0xeb08bfba);
        assert_eq!(fnv(b"http://primes.utm.edu/curios/home.php"), 0x62cc8e3d);
        assert_eq!(fnv(b"http://slashdot.org/"), 0xc3e20f5c);
        assert_eq!(
            fnv(b"http://tux.wr.usgs.gov/Maps/155.25-19.5.html"),
            0x39e97f17
        );
        assert_eq!(
            fnv(b"http://volcano.wr.usgs.gov/kilaueastatus.php"),
            0x7837b203
        );
        assert_eq!(
            fnv(b"http://www.avo.alaska.edu/activity/Redoubt.php"),
            0x319e877b
        );
        assert_eq!(fnv(b"http://www.dilbert.com/fast/"), 0xd3e63f89);
        assert_eq!(
            fnv(b"http://www.fourmilab.ch/gravitation/orbits/"),
            0x29b50b38
        );
        assert_eq!(fnv(b"http://www.fpoa.net/"), 0x5ed678b8);
        assert_eq!(fnv(b"http://www.ioccc.org/index.html"), 0xb0d5b793);
        assert_eq!(fnv(b"http://www.isthe.com/cgi-bin/number.cgi"), 0x52450be5);
        assert_eq!(fnv(b"http://www.isthe.com/chongo/bio.html"), 0xfa72d767);
        assert_eq!(fnv(b"http://www.isthe.com/chongo/index.html"), 0x95066709);
        assert_eq!(
            fnv(b"http://www.isthe.com/chongo/src/calc/lucas-calc"),
            0x7f52e123
        );
        assert_eq!(
            fnv(b"http://www.isthe.com/chongo/tech/astro/venus2004.html"),
            0x76966481
        );
        assert_eq!(
            fnv(b"http://www.isthe.com/chongo/tech/astro/vita.html"),
            0x063258b0
        );
        assert_eq!(
            fnv(b"http://www.isthe.com/chongo/tech/comp/c/expert.html"),
            0x2ded6e8a
        );
        assert_eq!(
            fnv(b"http://www.isthe.com/chongo/tech/comp/calc/index.html"),
            0xb07d7c52
        );
        assert_eq!(
            fnv(b"http://www.isthe.com/chongo/tech/comp/fnv/index.html"),
            0xd0c71b71
        );
        assert_eq!(
            fnv(b"http://www.isthe.com/chongo/tech/math/number/howhigh.html"),
            0xf684f1bd
        );
        assert_eq!(
            fnv(b"http://www.isthe.com/chongo/tech/math/number/number.html"),
            0x868ecfa8
        );
        assert_eq!(
            fnv(b"http://www.isthe.com/chongo/tech/math/prime/mersenne.html"),
            0xf794f684
        );
        assert_eq!(
            fnv(b"http://www.isthe.com/chongo/tech/math/prime/mersenne.html#largest"),
            0xd19701c3
        );
        assert_eq!(
            fnv(b"http://www.lavarnd.org/cgi-bin/corpspeak.cgi"),
            0x346e171e
        );
        assert_eq!(fnv(b"http://www.lavarnd.org/cgi-bin/haiku.cgi"), 0x91f8f676);
        assert_eq!(
            fnv(b"http://www.lavarnd.org/cgi-bin/rand-none.cgi"),
            0x0bf58848
        );
        assert_eq!(
            fnv(b"http://www.lavarnd.org/cgi-bin/randdist.cgi"),
            0x6317b6d1
        );
        assert_eq!(fnv(b"http://www.lavarnd.org/index.html"), 0xafad4c54);
        assert_eq!(
            fnv(b"http://www.lavarnd.org/what/nist-test.html"),
            0x0f25681e
        );
        assert_eq!(fnv(b"http://www.macosxhints.com/"), 0x91b18d49);
        assert_eq!(fnv(b"http://www.mellis.com/"), 0x7d61c12e);
        assert_eq!(
            fnv(b"http://www.nature.nps.gov/air/webcams/parks/havoso2alert/havoalert.cfm"),
            0x5147d25c
        );
        assert_eq!(
            fnv(b"http://www.nature.nps.gov/air/webcams/parks/havoso2alert/timelines_24.cfm"),
            0x9a8b6805
        );
        assert_eq!(fnv(b"http://www.paulnoll.com/"), 0x4cd2a447);
        assert_eq!(fnv(b"http://www.pepysdiary.com/"), 0x1e549b14);
        assert_eq!(
            fnv(b"http://www.sciencenews.org/index/home/activity/view"),
            0x2fe1b574
        );
        assert_eq!(fnv(b"http://www.skyandtelescope.com/"), 0xcf0cd31e);
        assert_eq!(fnv(b"http://www.sput.nl/~rob/sirius.html"), 0x6c471669);
        assert_eq!(fnv(b"http://www.systemexperts.com/"), 0x0e5eef1e);
        assert_eq!(
            fnv(b"http://www.tq-international.com/phpBB3/index.php"),
            0x2bed3602
        );
        assert_eq!(
            fnv(b"http://www.travelquesttours.com/index.htm"),
            0xb26249e0
        );
        assert_eq!(
            fnv(b"http://www.wunderground.com/global/stations/89606.html"),
            0x2c9b86a4
        );
        assert_eq!(fnv(&repeat_10(b"21701")), 0xe415e2bb);
        assert_eq!(fnv(&repeat_10(b"M21701")), 0x18a98d1d);
        assert_eq!(fnv(&repeat_10(b"2^21701-1")), 0xb7df8b7b);
        assert_eq!(fnv(&repeat_10(b"\x54\xc5")), 0x241e9075);
        assert_eq!(fnv(&repeat_10(b"\xc5\x54")), 0x063f70dd);
        assert_eq!(fnv(&repeat_10(b"23209")), 0x0295aed9);
        assert_eq!(fnv(&repeat_10(b"M23209")), 0x56a7f781);
        assert_eq!(fnv(&repeat_10(b"2^23209-1")), 0x253bc645);
        assert_eq!(fnv(&repeat_10(b"\x5a\xa9")), 0x46610921);
        assert_eq!(fnv(&repeat_10(b"\xa9\x5a")), 0x7c1577f9);
        assert_eq!(fnv(&repeat_10(b"391581216093")), 0x512b2851);
        assert_eq!(fnv(&repeat_10(b"391581*2^216093-1")), 0x76823999);
        assert_eq!(fnv(&repeat_10(b"\x05\xf9\x9d\x03\x4c\x81")), 0xc0586935);
        assert_eq!(fnv(&repeat_10(b"FEDCBA9876543210")), 0xf3415c85);
        assert_eq!(
            fnv(&repeat_10(b"\xfe\xdc\xba\x98\x76\x54\x32\x10")),
            0x0ae4ff65
        );
        assert_eq!(fnv(&repeat_10(b"EFCDAB8967452301")), 0x58b79725);
        assert_eq!(
            fnv(&repeat_10(b"\xef\xcd\xab\x89\x67\x45\x23\x01")),
            0xdea43aa5
        );
        assert_eq!(fnv(&repeat_10(b"0123456789ABCDEF")), 0x2bb3be35);
        assert_eq!(
            fnv(&repeat_10(b"\x01\x23\x45\x67\x89\xab\xcd\xef")),
            0xea777a45
        );
        assert_eq!(fnv(&repeat_10(b"1032547698BADCFE")), 0x8f21c305);
        assert_eq!(
            fnv(&repeat_10(b"\x10\x32\x54\x76\x98\xba\xdc\xfe")),
            0x5c9d0865
        );
        assert_eq!(fnv(&repeat_500(b"\x00")), 0xfa823dd5);
        assert_eq!(fnv(&repeat_500(b"\x07")), 0x21a27271);
        assert_eq!(fnv(&repeat_500(b"~")), 0x83c5c6d5);
        assert_eq!(fnv(&repeat_500(b"\x7f")), 0x813b0881);
    }
}
