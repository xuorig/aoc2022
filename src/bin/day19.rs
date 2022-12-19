use std::collections::HashMap;

use nom::{character::complete::digit1, bytes::complete::tag, sequence::delimited, IResult, combinator::map_res};

fn main() {
    let input = include_str!("../inputs/19.txt");
    let blueprints: Vec<Blueprint> = input.trim().lines().map(|l| parse_blueprint(l).unwrap().1).collect();
    part1(&blueprints);
    part2(&blueprints);
}

fn part1(blueprints: &Vec<Blueprint>) {
    let sum: i32 = blueprints.iter().map(|bp| {
        bp.id * evaluate_blueprint((0,0,0,0,1,0,0,0,24), bp, &mut HashMap::new())
    }).sum();

    println!("{}", sum);
}

fn part2(blueprints: &Vec<Blueprint>) {
    let product: i32 = blueprints.iter().take(3).map(|bp| {
        evaluate_blueprint((0,0,0,0,1,0,0,0,32), bp, &mut HashMap::new())
    }).product();

    println!("{}", product);
}

type State = (i32, i32, i32, i32, i32, i32, i32, i32, i32);

fn evaluate_blueprint(state: State, blueprint: &Blueprint, cache: &mut HashMap<State, i32>) -> i32 {
    let (
        ore,
        clay,
        obs,
        geo,
        ore_d,
        clay_d,
        obs_d,
        geo_d,
        time
        ) = state;

    if time == 0 {
        return geo;
    }

    if cache.get(&state).is_some() {
        return cache[&state];
    }

    let new_ore = ore + ore_d;
    let new_clay = clay + clay_d;
    let new_obs = obs + obs_d;
    let new_geo = geo + geo_d;

    let mut geodes = vec![];

    // If we have enough ore and obs we could buy a geo bot.
    if ore >= blueprint.geo_bot.0 && obs >= blueprint.geo_bot.1 {
        let max = evaluate_blueprint((
                    new_ore - blueprint.geo_bot.0,
                    new_clay,
                    new_obs - blueprint.geo_bot.1,
                    new_geo,
                    ore_d,
                    clay_d,
                    obs_d,
                    geo_d + 1,
                    time - 1
                    ), blueprint, cache);
        cache.insert(state, max);
        return max
    }

    // If we have enough ore and clay we could buy a obs bot.
    if ore >= blueprint.obs_bot.0 && clay >= blueprint.obs_bot.1 {
        geodes.push(evaluate_blueprint((
                    new_ore - blueprint.obs_bot.0,
                    new_clay - blueprint.obs_bot.1,
                    new_obs,
                    new_geo,
                    ore_d,
                    clay_d,
                    obs_d + 1,
                    geo_d,
                    time - 1
                    ), blueprint, cache));
    }

    // If we have enough ore we could buy an ore bot.
    if ore >= blueprint.ore_bot {
        geodes.push(evaluate_blueprint((
                    new_ore - blueprint.ore_bot,
                    new_clay,
                    new_obs,
                    new_geo,
                    ore_d + 1, clay_d,
                    obs_d,
                    geo_d,
                    time - 1
                    ), blueprint, cache))
    }

    // If we have enough ore we could buy a clay bot.
    if ore >= blueprint.clay_bot {
        geodes.push(evaluate_blueprint((
                    new_ore - blueprint.clay_bot,
                    new_clay,
                    new_obs,
                    new_geo,
                    ore_d,
                    clay_d + 1,
                    obs_d,
                    geo_d,
                    time - 1
                    ), blueprint, cache))
    }

 
    // We can wait if theres nothing else to do
    // or if we have a positive geode mining
    if geodes.is_empty() || geo_d > 0 {
        geodes.push(
            evaluate_blueprint(
                (
                    new_ore,
                    new_clay,
                    new_obs,
                    new_geo,
                    ore_d,
                    clay_d,
                    obs_d,
                    geo_d,
                    time - 1
                ), 
                blueprint,
                cache
                )
            );
    }


    let max = geodes.iter().max().unwrap();
    cache.insert(state, *max);
    *max
}

fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    let (i, id) = map_res(delimited(tag("Blueprint "), digit1, tag(": ")), str::parse::<i32>)(input)?;
    let (i, ore_bot_ore_cost) = map_res(delimited(tag("Each ore robot costs "), digit1, tag(" ore. ")), str::parse::<i32>)(i)?;
    let (i, clay_bot_ore_cost) = map_res(delimited(tag("Each clay robot costs "), digit1, tag(" ore. ")), str::parse::<i32>)(i)?;
    let (i, obs_bot_ore_cost) = map_res(delimited(tag("Each obsidian robot costs "), digit1, tag(" ore ")), str::parse::<i32>)(i)?;
    let (i, obs_bot_clay_cost) = map_res(delimited(tag("and "), digit1, tag(" clay. ")), str::parse::<i32>)(i)?;
    let (i, geo_bot_ore_cost) = map_res(delimited(tag("Each geode robot costs "), digit1, tag(" ore ")), str::parse::<i32>)(i)?;
    let (i, geo_bot_obs_cost) = map_res(delimited(tag("and "), digit1, tag(" obsidian.")), str::parse::<i32>)(i)?;

    return Ok((
    i,
        Blueprint {
            id,
            ore_bot: ore_bot_ore_cost,
            clay_bot: clay_bot_ore_cost,
            obs_bot: (obs_bot_ore_cost, obs_bot_clay_cost),
            geo_bot: (geo_bot_ore_cost, geo_bot_obs_cost)
        }
    ))
}

#[derive(Debug)]
struct Blueprint {
    id: i32,
    ore_bot: i32,
    clay_bot: i32,
    obs_bot: (i32, i32),
    geo_bot: (i32, i32),
}
