if !cargo -v
then 
    echo "You must have cargo installed to build this application";
    exit 1;
fi 
cargo build;
mv target/debug/port-scanner /usr/local/bin/oxiscanner;

