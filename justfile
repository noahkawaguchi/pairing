bind-addr := 'localhost:4000'

serve:
    cargo run --release

s key val:
    curl --http0.9 {{ bind-addr }}/set?{{ key }}={{ val }}

g key:
    curl --http0.9 {{ bind-addr }}/get?key={{ key }}
