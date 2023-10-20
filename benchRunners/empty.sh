count=1000

echo "starting empty"

#   Node
node ./benchmarks/empty/javascript/bench.js $count
sleep 5s
bash utils/append_to_latest_csv.sh "NodeEmpty"

#   Python
pypy ./benchmarks/empty/python/bench.py $count
sleep 5s
bash utils/append_to_latest_csv.sh "PythonEmpty"

#   Pypy
pypy ./benchmarks/empty/python/bench.py $count
sleep 5s
bash utils/append_to_latest_csv.sh "PypyEmpty"

#   C#
dotnet run --project ./benchmarks/empty/csharp/Empty.csproj --configuration Release $count
sleep 5s
bash utils/append_to_latest_csv.sh "CsharpEmpty"

#   Java
java --enable-native-access=ALL-UNNAMED --enable-preview --source 21 ./benchmarks/empty/java/Bench.java $count
sleep 5s
bash utils/append_to_latest_csv.sh "JavaEmpty"

#   C
gcc benchmarks/empty/c/bench.c -O3 -o benchmarks/empty/c/bench -L./target/release -lrapl_lib -Wl,-rpath=./target/release && ./benchmarks/empty/c/bench $count
sleep 5s
bash utils/append_to_latest_csv.sh "Cempty"
