use std::process::Command;

#[derive(Debug)]
enum ScreenRotation
{
    Right,
    Left,
    Normal,
    Inverted
}

#[derive(Debug)]
struct PointPosition
{
    width : u32,
    high: u32
}

fn main()
{

    let monitor_info_cli = String::from_utf8(
        Command::new("xrandr")
            .args(&["--prop"])
            .output()
            .unwrap()
            .stdout,
    ).unwrap();

    //println!("{}", monitor_info_cli);
    //	EDID:
    //
    //
    //
    let lines = monitor_info_cli.split("\n");

    for line in lines
    {
        if line.len() == 0 { continue; }
        if !(line.chars().nth(0 as usize).unwrap().eq(&' ') || line.chars().nth(0 as usize).unwrap().eq(&'\t'))
        {
            println!("{}", line);
            let comps : Vec<&str>= line
                .split(&['(', ')'][..])
                .filter( |x| x != &"")
                .collect();

            if comps.len() == 2
            {
                let name_and_mode = *comps.first().unwrap();
            }

            if comps.len() == 3
            {
                let name_and_mode = *comps.first().unwrap();
                let monitor_size = *comps.last().unwrap();
            }

            for comp in comps
            {
                println!("{}", comp);
            }
        }
    }
}

fn parse_name_and_mode(value : &str) -> (String, bool, Option<ScreenRotation>, Option<PointPosition>, Option<PointPosition>)
{
    let comps : Vec<&str> = value.split(" ")
        .collect();

    let name = String::from(comps.first().unwrap());

    if comps.len() == 2
    {

    }
    else if comps.len() == 3
    {

    }
    else if comps.len() == 4
    {

    }
}
