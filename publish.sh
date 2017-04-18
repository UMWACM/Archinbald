cargo build --release;
rsync -I --progress ./target/release/archinbald cbaxter@cs.umw.edu:~;
rsync -I --progress ./res/quotes.txt cbaxter@cs.umw.edu:~;
rsync -I --progress config.json cbaxter@cs.umw.edu:~;
ssh cbaxter@cs.umw.edu 'pkill archinbald; chmod go-rwx config.json; nohup ~/archinbald';
