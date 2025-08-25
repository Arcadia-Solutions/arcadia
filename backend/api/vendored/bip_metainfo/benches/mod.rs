#![feature(test)]

extern crate bip_metainfo;
extern crate test;

#[cfg(test)]
mod benches {
    use bip_metainfo::{Metainfo, MetainfoBuilder, DirectAccessor};
    use test::Bencher;

    #[bench]
    fn bench_build_multi_kb_metainfo(b: &mut Bencher) {
        let file_content = vec![55u8; 10 * 1024 * 1024];

        b.iter(|| {
            let direct_accessor = DirectAccessor::new("100MBFile", &file_content);

            MetainfoBuilder::new().build(2, direct_accessor, |_| ()).unwrap();
        });
    }

    #[bench]
    fn bench_parse_multi_kb_metainfo(b: &mut Bencher) {
        let metainfo = include_bytes!("multi_kb.metainfo");

        b.iter(|| Metainfo::from_bytes(&metainfo[..]).unwrap());
    }
}