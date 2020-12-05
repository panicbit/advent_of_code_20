set -x

event=2020
start=1
end=25

for day in `seq -f "%02.f" $start $end`; do
    for part in `seq 1 2`; do
        if [ "$part" == "1" ]; then
            project="day${day}"
        else
            project="day${day}_${part}"
        fi

        cargo new --bin "$project"
        (
            cd "$project"
            cargo add aoc
            cat << EOF > src/main.rs
#[macro_use] extern crate aoc;

#[aoc($event, $day, $part)]
fn main(input: &str) -> i32 {
    unimplemented!()
}
EOF
        )&
    done
done
wait
