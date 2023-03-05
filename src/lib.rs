//        .,,.
//        ,KMKkkkkkkkklll. ..                             ..       .,,,.
//     .lkKKl,,,,,,,lkkKMKkKkll. ..                 ..,l,lKKkkkkkkkKMMKl.
//    .lKKl.           .,,lkkKMKkKk,. ,l.  ,l. ,l. .kKKKkkl,,,,,,,,,lkKMKkl.
//  .,kMk.   .,,,,,.         .,,lkKMKkKMKkkKMklKMKkKKl,.       .,,,.  .,kMKl.
//  .kMk.    .,lkKMKkkl.        ...,lKMMk,llkMMklKKl.  .... .lkKKl,.    .kMl
//  .kMl         .,,lKMKkl.  .. lKl. .ll.   .ll. ..  ,lkKKklKMKl.        lmk.
// .kMk.             .lKMMKllKKkKKKl                .kMKKMMMMMl..        lmk.
//  ,KK,              .KMKKKKklkKl..                 ,l..,lKKKKKx,l,    ,KKl.
// .lKK,          ..,,lMK,....  ..                         ..lKkKMMl .. lMKl.
//  lMl         ..lKKKKMK,                                   .. lKKKlkl .kMk.
// .kMl         lKKKKMk,.                                       ...lKMK, lMk.
//  lMk.      ,lkMk.,l.                                             ,lkl,KK,
//  ,KMl     .kkll,                     .,.                ..         ..,Kk,.
//  .kMk.    lMl      .lkkkkkkkl,,.    ,KMl                lk,     .lkkkkKMMK,
//   ,KMl    ,l.     .lklkMk,,,lKMKkl,lKMMl                lMKl,,lkkkl,,kMMMMk.
//    lMK.               lMl    .,lkkKMKl,.                .lKMMKkl.   .kMKKMK,
//    ,KMKl.             lMk.       .kMl                     ,KMk.  .,lKKl.,KMk.
//  .,lKKkl.             ,KMKl,,,,,lKKl.      .lkkkkkkkkkl.   ,KMKkkKKkl.  lMK,
//  ,KMKk,.               .lkkkkkkkkl.        lMMMMMMMMMMMl    .,,,,,.     ,KMk.
//  .lKMMK,                                   .kMMMMMMMMMMl                 lMMl
//   .lkKMk..l,                .ll,.           .lKMMMMMMkl.       .,ll.     lMK.
//  .,,lKKl.lMKkl.            ,KK,               .,lKMKl.           ,KK,    lMK,
//  .lKMMk,..,,,,.            lMl                  .kMl              lMl   ,KMMK,
//    ,KMMK.   ,Kl            lMl          .,,,,,lkKMMKkl,,.         lMl   lMKkl.
//   .kMkl.   ,KK,            lMl          .kMMMMKKMMMMKKMMKl.      ,KMl   lMKl.
//   .lkKKkkl.lMKkkl.         ,KKl.         .lkKMkkMKKMkkMk,.     .lKMk.   .kMMl
//      .lKMMl.,,,,. ..        ,KMKkl.         .lKKl..lKKl.      .kKkl.   ,kKMK,
//       .kMMKk,    .kl.,,.     .,,lkl.   ..     ..    ..  ..   .ll.   .,,kMMk.
//      .lKMMMKl,,,.lMKKKl.               lKl.           .lKl          lMKKMK,
//     .kMMKkkkkkKMl.lkk,                 .kMKl,.     .,lKKl.       .lkKMl.,.
//    .lKMMk.    lMKkKMMKk.,,              .lkKMKkkkkkKMKl.      ,k,lKKMK,
//      ,KKl. .. .,,,,kMKl.lK, .,,.           .,,,,,,,,,.   ,;,lkKMKk,.,.
//    .lKMk. .kl.l,   .,.  lMKkKKKklkk,.,,,,.             .,kMMKKMk,.
//    .lKMK,.kMKkk,        ,kl,,.lMKlkKKKkKMK,.,,lllllkkkkkKKll,lMl
//    .lKMk..,,,.                .,. lKl. .ll.lKKMKkl,lKKl...  .kMKl.
//   .lKMk.                   ,l. .. ..       ...l,    ..     ,KMKl,.
//   ,KMKl.  ..               lMklKl                          lMMk.
//  .lKMkl.  lk.,l.  .lx,     ,kMKl.
//  .lKk.    lMKKMl lKKl

use anyhow::Result;
use board::Board;

pub mod board;
pub mod card;
pub mod currency;
pub mod playing;
pub mod rules;
pub mod traits;
pub mod util;

pub struct Game {
    pub board: Box<[Board]>,
    pub rules: rules::Rules,
}

impl Game {
    pub fn new_with_rules(rules: rules::Rules, decks: Vec<Vec<card::Card>>) -> Result<Self> {
        Ok(Self {
            board: decks.into_iter().map(|deck| Board::new(&rules, deck)).collect::<Result<Vec<Board>>>()?.into_boxed_slice(),
            rules,
        })
    }
    pub fn new_with_preset(preset: rules::Preset, decks: Vec<Vec<card::Card>>) -> Result<Self> {
        Self::new_with_rules(rules::Rules::new(preset), decks)
    }
}
