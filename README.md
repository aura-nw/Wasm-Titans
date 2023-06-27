# Contracts for aura cars

## 1. Introduction

**Reference**: https://golden-racer-395.notion.site/0xTitans-Rules-guide-8b8c959a2b4e4313a57b186003b32543

Each game involves 3 cars (3 players), and each car is a smart contract for which you have to design the best strategy before the race to win the game.

**The winner is the first car that reaches the final line: a Y position of 1000 (or greater)** 

You will have to manage properly your resources for whether accelerating or firing a shell (more details below) which means designing the best strategy.

It is more a **theory/resource management** than a pure solidity **coding-skills-**based game (the level of solidity required in order to properly play is low and training is easy)


**Possible actions:**

- **ACCELERATE:**
    - Increase speed to +1.
    - Speed never decreases unless you are hit by a shell
- **SHELL:**
    - Reduces speed of the car in front back to 1 (like the original version)
    - Can be used forward only
    - Can be used to shoot down bananas in the way
    - Takes effect instantly (not like projectiles)
- **SUPER_SHELL:**
    - Same as a shell but touches every car beyond until the n°1 (and it’s more expensive)
    - It is NOT CLEARING all the bananas in the way (like in the last edition)
    - Going through shields
- **BANANA**:
    - Drop on the road and the next car that goes through get his speed divided by 2
    - Stops the car at the position of the banana
- **SHIELD**:
    - Protects the car from getting a shell. One shield = 1-time protection
    - Decrease by one after each turn

## 2. Setup
* Install rust


## 3. Write your car contract
We have write 3 cars examples for demo the racing.

**See contract examples in `contracts/car-*`**

* Default config:
    * number_players = 3
    * post_sell_speed = 0
    * init_balance = 17500
    * target_distance = 1000
    * banana_speed_modifier = 0
    * shell_target_price = 0,
    * shell_per_turn_decrease = 0,
    * shell_sell_per_turn = 0,
    * accel_target_price = 0,
    * accel_per_turn_decrease = 0,
    * accel_sell_per_turn = 0,
    * ss_target_price = 0,
    * ss_per_turn_decrease = 0,
    * ss_sell_per_turn = 0,
    * banana_target_price = 0,
    * banana_per_turn_decrease = 0,
    * banana_sell_per_turn = 0,
    * shield_target_price = 0,
    * shield_per_turn_decrease = 0,
    * shield_sell_per_turn = 0,


## 4. Run test
**Run script**
    
```
# Test your code
cargo test

# Run the racing with mock
./devtools/run-test.sh
```

The results will show winer car and actions of each car when they
## 5. Deploy on chain