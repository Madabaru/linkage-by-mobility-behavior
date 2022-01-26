cargo build --release && ./target/release/linkage-by-mobility-behavior --metric euclidean 
cargo build --release && ./target/release/linkage-by-mobility-behavior --metric manhattan
cargo build --release && ./target/release/linkage-by-mobility-behavior  --metric cosine
cargo build --release && ./target/release/linkage-by-mobility-behavior --metric non_intersection
cargo build --release && ./target/release/linkage-by-mobility-behavior --metric bhattacharyya
cargo build --release && ./target/release/linkage-by-mobility-behavior  --metric kullbrack_leibler
cargo build --release && ./target/release/linkage-by-mobility-behavior  --metric total_variation
cargo build --release && ./target/release/linkage-by-mobility-behavior  --metric jeffries_matusita
cargo build --release && ./target/release/linkage-by-mobility-behavior  --metric chi_quared

cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street 
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields location_code
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street postcode
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street postcode state
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street postcode state location_code
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields location_code day hour
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street postcode state location_code day hour
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street postcode state location_code day 
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street postcode state location_code hour
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street postcode state location_code hour day
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street postcode state village
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street postcode state hamlet
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street postcode state highway
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street postcode state suburb
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street postcode state village highway
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street postcode state village highway hamlet
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street postcode state village highway hamlet suburb
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street postcode state village highway hamlet suburb location_code
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street postcode state village highway hamlet suburb location_code hour day

cargo build --release && ./target/release/linkage-by-mobility-behavior  --click_trace_sample_size 1
cargo build --release && ./target/release/linkage-by-mobility-behavior  --click_trace_sample_size 2
cargo build --release && ./target/release/linkage-by-mobility-behavior  --click_trace_sample_size 3
cargo build --release && ./target/release/linkage-by-mobility-behavior  --click_trace_sample_size 4
cargo build --release && ./target/release/linkage-by-mobility-behavior  --click_trace_sample_size 5
cargo build --release && ./target/release/linkage-by-mobility-behavior  --click_trace_sample_size 6
cargo build --release && ./target/release/linkage-by-mobility-behavior  --click_trace_sample_size 7
cargo build --release && ./target/release/linkage-by-mobility-behavior  --click_trace_sample_size 8
cargo build --release && ./target/release/linkage-by-mobility-behavior  --click_trace_sample_size 9
cargo build --release && ./target/release/linkage-by-mobility-behavior  --click_trace_sample_size 10
cargo build --release && ./target/release/linkage-by-mobility-behavior  --click_trace_sample_size 15
cargo build --release && ./target/release/linkage-by-mobility-behavior  --click_trace_sample_size 20
cargo build --release && ./target/release/linkage-by-mobility-behavior  --click_trace_sample_size 25
cargo build --release && ./target/release/linkage-by-mobility-behavior  --click_trace_sample_size 50

cargo build --release && ./target/release/linkage-by-mobility-behavior  --client_sample_size 10
cargo build --release && ./target/release/linkage-by-mobility-behavior  --client_sample_size 20
cargo build --release && ./target/release/linkage-by-mobility-behavior  --client_sample_size 30
cargo build --release && ./target/release/linkage-by-mobility-behavior  --client_sample_size 40
cargo build --release && ./target/release/linkage-by-mobility-behavior  --client_sample_size 50
cargo build --release && ./target/release/linkage-by-mobility-behavior  --client_sample_size 100
cargo build --release && ./target/release/linkage-by-mobility-behavior  --client_sample_size 200
cargo build --release && ./target/release/linkage-by-mobility-behavior  --client_sample_size 400
cargo build --release && ./target/release/linkage-by-mobility-behavior  --client_sample_size 500
cargo build --release && ./target/release/linkage-by-mobility-behavior  --client_sample_size 1000

cargo build --release && ./target/release/linkage-by-mobility-behavior  --typical true --client_sample_size 2
cargo build --release && ./target/release/linkage-by-mobility-behavior  --client_sample_size 2
cargo build --release && ./target/release/linkage-by-mobility-behavior  --typical true --client_sample_size 5
cargo build --release && ./target/release/linkage-by-mobility-behavior  --client_sample_size 5
cargo build --release && ./target/release/linkage-by-mobility-behavior  --typical true --client_sample_size 10
cargo build --release && ./target/release/linkage-by-mobility-behavior  --client_sample_size 10
cargo build --release && ./target/release/linkage-by-mobility-behavior  --typical true --client_sample_size 20
cargo build --release && ./target/release/linkage-by-mobility-behavior  --client_sample_size 20
cargo build --release && ./target/release/linkage-by-mobility-behavior  --typical true --client_sample_size 40
cargo build --release && ./target/release/linkage-by-mobility-behavior  --client_sample_size 40

cargo build --release && ./target/release/linkage-by-mobility-behavior  --typical true --client_sample_size 2 --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --client_sample_size 2 --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --typical true --client_sample_size 5 --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --client_sample_size 5 --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --typical true --client_sample_size 10 --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --client_sample_size 10 --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --typical true --client_sample_size 20 --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --client_sample_size 20 --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --typical true --client_sample_size 40 --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --client_sample_size 40 --approach sequence

cargo build --release && ./target/release/linkage-by-mobility-behavior  --client_sample_size 10 --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --client_sample_size 20 --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --client_sample_size 30 --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --client_sample_size 40 --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --client_sample_size 50 --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --client_sample_size 100 --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --client_sample_size 200 --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --client_sample_size 400 --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --client_sample_size 500 --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --client_sample_size 1000 --approach sequence


cargo build --release && ./target/release/linkage-by-mobility-behavior  --click_trace_sample_size 1 --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --click_trace_sample_size 2 --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --click_trace_sample_size 3 --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --click_trace_sample_size 4 --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --click_trace_sample_size 5 --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --click_trace_sample_size 6 --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --click_trace_sample_size 7 --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --click_trace_sample_size 8 --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --click_trace_sample_size 9 --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --click_trace_sample_size 10 --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --click_trace_sample_size 15 --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --click_trace_sample_size 20 --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --click_trace_sample_size 25 --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --click_trace_sample_size 50 --approach sequence

cargo build --release && ./target/release/linkage-by-mobility-behavior  --approach sequence --scope local
cargo build --release && ./target/release/linkage-by-mobility-behavior  --approach sequence --scope local --strategy nw
cargo build --release && ./target/release/linkage-by-mobility-behavior  --approach sequence --scope global
cargo build --release && ./target/release/linkage-by-mobility-behavior  --approach sequence --scope global --strategy nw

cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields location_code --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street postcode --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street postcode state --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street postcode state location_code --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields location_code day hour --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street postcode state location_code day hour --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street postcode state location_code day --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street postcode state location_code hour --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street postcode state location_code hour day --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street postcode state village --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street postcode state hamlet --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street postcode state highway --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street postcode state suburb --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street postcode state village highway --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street postcode state village highway hamlet --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street postcode state village highway hamlet suburb --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street postcode state village highway hamlet suburb location_code --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior  --fields street postcode state village highway hamlet suburb location_code hour day --approach sequence

cargo build --release && ./target/release/linkage-by-mobility-behavior --reverse false --approach sequence
cargo build --release && ./target/release/linkage-by-mobility-behavior --reverse true --approach sequence

