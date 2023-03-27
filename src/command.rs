use std::io::{stdin,stdout,Write};

pub fn effect() {

   let mut type_input = String::new();
   let mut secs_input = String::new();
   let mut amp_input = String::new();
   let mut par_input = String::new();

   println!("\n [1] Absorption");
   println!(" [2] Bad Luck");
   println!(" [3] Bad Omen");
   println!(" [4] Blindness");
   println!(" [5] Conduit Power");
   println!(" [6] Darkness");
   println!(" [7] Dolphins Grace");
   println!(" [8] Fire Resistance");
   println!(" [9] Glowing");
   println!(" [10] Haste");
   println!(" [11] Health Boost");
   println!(" [12] Hero of the Village");
   println!(" [13] Hunger");
   println!(" [14] Instant Damage");
   println!(" [15] Instant Health");
   println!(" [16] Invisibility");
   println!(" [17] Jump Boost");
   println!(" [18] Levitation");
   println!(" [19] Luck");
   println!(" [20] Mining Fatigue");
   println!(" [21] Nausea");
   println!(" [22] Night Vision");
   println!(" [23] Poision");
   println!(" [24] Regeneration");
   println!(" [25] Resistance");
   println!(" [26] Saturation");
   println!(" [27] Slow Falling");
   println!(" [28] Slowness");
   println!(" [29] Speed");
   println!(" [30] Strength");
   println!(" [31] Water Breathing");
   println!(" [32] Weakness");
   println!(" [33] Wither");

   print!("\n >> ");

   let _ = stdout().flush();
   stdin().read_line(&mut type_input)
       .expect("Did not enter a string.");
   let effect_choice: i32 = type_input.trim().parse()
       .expect("Please enter a number");


   print!("\n Enter the duration of the effect (0-1000000):  ");

   let _ = stdout().flush();
   stdin().read_line(&mut secs_input)
       .expect("Did not enter a string.");
   let mut secs: i32 = secs_input.trim().parse()
       .expect("Please enter a number");

   if secs <= 0 { 

      secs = 60;
   } else if secs > 1000000 {

      secs = 1000000;
   }


   print!("\n Enter the ampifier of the effect (1-255):  ");

   let _ = stdout().flush();
   stdin().read_line(&mut amp_input)
       .expect("Did not enter a string.");
   let mut amp: i32 = amp_input.trim().parse()
       .expect("Please enter a number");

   if amp <= 1 { 

      amp = 2;
   } else if amp > 255 {

      amp = 255;
   }


   print!("\n Hide particles? (y/n): ");
   let _ = stdout().flush();
   let _b1 = std::io::stdin().read_line(&mut par_input).unwrap();

   match par_input.as_str().trim() {

      "y" => par_input = String::from("true"),
      "n" => par_input = String::from("false"),
      _ => par_input = String::from("false")
   }


   match effect_choice {

      1 => println!("\n /effect give @p absorption {} {} {}", secs, amp, par_input),
      2 => println!("\n /effect give @p unluck {} {} {}", secs, amp, par_input),
      3 => println!("\n /effect give @p bad_omen {} {} {}", secs, amp, par_input),
      4 => println!("\n /effect give @p blindness {} {} {}", secs, amp, par_input),
      5 => println!("\n /effect give @p conduit_power {} {} {}", secs, amp, par_input),
      6 => println!("\n /effect give @p darkness {} {} {}", secs, amp, par_input),
      7 => println!("\n /effect give @p dolphins_grace {} {} {}", secs, amp, par_input),
      8 => println!("\n /effect give @p fire_resistance {} {} {}", secs, amp, par_input),
      9 => println!("\n /effect give @p glowing {} {} {}", secs, amp, par_input),
      10 => println!("\n /effect give @p haste {} {} {}", secs, amp, par_input),
      11 => println!("\n /effect give @p health_boost {} {} {}", secs, amp, par_input),
      12 => println!("\n /effect give @p hero_of_the_village {} {} {}", secs, amp, par_input),
      13 => println!("\n /effect give @p hunger {} {} {}", secs, amp, par_input),
      14 => println!("\n /effect give @p instant_damage {} {} {}", secs, amp, par_input),
      15 => println!("\n /effect give @p instant_health {} {} {}", secs, amp, par_input),
      16 => println!("\n /effect give @p invisibility {} {} {}", secs, amp, par_input),
      17 => println!("\n /effect give @p jump_boost {} {} {}", secs, amp, par_input),
      18 => println!("\n /effect give @p levitation {} {} {}", secs, amp, par_input),
      19 => println!("\n /effect give @p luck {} {} {}", secs, amp, par_input),
      20 => println!("\n /effect give @p mining_fatigue {} {} {}", secs, amp, par_input),
      21 => println!("\n /effect give @p nausea {} {} {}", secs, amp, par_input),
      22 => println!("\n /effect give @p night_vision {} {} {}", secs, amp, par_input),
      23 => println!("\n /effect give @p poison {} {} {}", secs, amp, par_input),
      24 => println!("\n /effect give @p regeneration {} {} {}", secs, amp, par_input),
      25 => println!("\n /effect give @p resistance {} {} {}", secs, amp, par_input),
      26 => println!("\n /effect give @p saturation {} {} {}", secs, amp, par_input),
      27 => println!("\n /effect give @p slow_falling {} {} {}", secs, amp, par_input),
      28 => println!("\n /effect give @p slowness {} {} {}", secs, amp, par_input),
      29 => println!("\n /effect give @p speed {} {} {}", secs, amp, par_input),
      30 => println!("\n /effect give @p strength {} {} {}", secs, amp, par_input),
      31 => println!("\n /effect give @p water_breathing {} {} {}", secs, amp, par_input),
      32 => println!("\n /effect give @p weakness {} {} {}", secs, amp, par_input),
      33 => println!("\n /effect give @p wither {} {} {}", secs, amp, par_input),
      _ => println!("\n /effect give @p absorption {} {} {}", secs, amp, par_input)

   }

}