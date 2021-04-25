use colored::*;
use users;

const GREEN: (u8, u8, u8) = (149, 255, 164); // !
const DARK_BROWN: (u8, u8, u8) = (175, 95, 0); // &
const LIGHT_BROWN: (u8, u8, u8) = (255, 233, 170); // *

const WOODO_STRING: &str = "&                                         @@6@
                                        @C6CC@
                        O@C#           RCCCC6@
                       KCGCCCO@      /CCCCOe@
                       O66CCCCCC@   @CCCCeK
                        @ReeOCCCCCCCCCCGe/
                             #@eeCCCCCe@
                                  eCCe@
!    @((6666@&                      /CCe/
!   %((((6666O&                      KCG/
!  @(((((666##@     OO/&           #@@CC@C6@/
!  %((((6666###  @((((666/&     RCCCCOCCKeeCCC7
!  @6666666###@ ((((((66O#@&   CCCCCCCCCCCCCCCe
!   666666O### @6%(((666###&   CCCCCCCCCCCCCCCeO!                   #@O
    @######@  #6666666####&   CCCCCW &CCCCCW &CCCe@!                @((((((6@y
&          #ee!7O66666S####R&   CCCCCB &CCCCCB &CCCG@!               (6(((((((6@
  @(66666@& eeee!@########@&    7CCCCCCCCCCCCCC6K!              @66((((((%66~
 %((6666##@&eeeeee!@####@&      R7CCB         &CCCe!      /@@@7   @666((((666#O
@((((666###@&eeeee            @(CCCCCCCCCCCCCCe!    @(((((66@  ##O66666O#@
(6666666###R&eeeeC@           #(CCCCCCCCCCCCC6e!   @6(((((%6#@  R#######@
@666666####@&  ~eeC@          O7CCCCCCCCCCCCCee!   666(((66###&  @e!@@R@O
 O6666####R&     7eeCC@       @CCCCCCCCCCCCCCeR!   R666666S###&eeee!  @%(O@
  @S#####@&        KeeCCCS@##@RCCCCCCCCCCCCCCe@!    66666####&Reeee!%((((666@
&                    @eeee6CCCCCCCCCCCCCCCCCeeS7!    @#####@&eeeee!(66((6666#O
&                       OeeeeCCCCCCCCCCCCCCCeeeCC@      @Oeee@#!@66666666##@
&                          Re7CCCC*@K&CCCCCCCeeeeeCCCCCCCCCOR7!    66666666##@
&                           @CCCCC*(@&CCCCCCGe@eeeeeeeeeee@!       @##S666###
&                          OCCCCC*R(6&CCCCCCeeRe@@O~O~!             /@######
&                          CCCCCC*6K&CCCCCCeeK
                         @C*(66&CCCCCCC*K&CCee/
                        @CC*6(&CCCCCCC*SRC&ee@
                        CC*S(R&CCCCCCO*(66&ee
                       @CC*@@&CCCCCCC*6CC&ee@
                      #CCCCCCCCCCCCCCCee
                      CCCCCCCCCC*R@&CCCee@
                     ~CCCCCCCCC*S(@&CCCeee
                     RCCCCCCCC*S((&CCCGee
                    ~CCCCCCCCC*S(G&CCCee@
                    @CC*K(&CCCCC*6G&CCCCee@
                    @CC*@(@&CCCC*@&CCCCCeee@
                  @C6CC*G(@&CCCCCCCCCCeeee~
                  CCKCC*@6@&CCCCCCCKCCCeee@
                  CO@CC*e6&CCCCCCCCCCCCCee@
                 /Ce@CCCCCCCCCCCCSeCC6eS@
                 CCCe@CCCCCCCCCCCKe@eeee/
                @CeeeO/RCCCCCCOeeeeeKeeee@
                7Oee~     @KeeeeeR@  @eeeee@@OK@O
             @@eeeeeK                   @eeeCCCCCCC@
          @CCCCeeeeeee                     @CCCCCCCC@
        OCCCCCCCCCee@                        CCCCCCCC6
          ~#@@@#~                             @CCCCCee#
                                                 @Ree@";

fn main() {
    let uid = users::get_current_uid();
    let euid = users::get_effective_uid();

    if uid != 0 || euid != 0 {
        println!("It's a weird tree.");
        print_woodo();
    } else {
        print_woodo();
    }
}

fn print_woodo() {
    let mut background = "None";
    let mut last = 0;
    let mut to_color = DARK_BROWN;

    for (index, separator) in WOODO_STRING
        .match_indices(|c: char| (c == '!' || c == '&' || c == '*' || c == 'W' || c == 'B'))
    {
        if last != index {
            if background != "white" && background != "black" {
                print!(
                    "{}",
                    &WOODO_STRING[last..index].truecolor(to_color.0, to_color.1, to_color.2)
                );
            } else {
                print!("{}", &WOODO_STRING[last..index].on_color(background));
                background = "None";
            }
        }

        last = index + 1;

        if separator == "!" {
            to_color = GREEN;
        } else if separator == "&" {
            to_color = DARK_BROWN;
        } else if separator == "*" {
            to_color = LIGHT_BROWN;
        } else if separator == "W" {
            background = "white";
        } else if separator == "B" {
            background = "black";
        }
    }

    println!(
        "{}",
        &WOODO_STRING[last..].truecolor(to_color.0, to_color.1, to_color.2)
    );
}
