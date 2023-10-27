testName="sleep"
folder="sleep"
count=1
sleep_time=5

echo "!!! Starting $testName !!!"
echo

#   C
echo --- Starting C ---
gcc benchmarks/$folder/c/bench.c -O3 -o benchmarks/$folder/c/bench -L./target/release -lrapl_lib -Wl,-rpath=./target/release && ./benchmarks/$folder/c/bench $count $sleep_time
sleep 5s
bash utils/append_to_latest_csv.sh "C$testName"
echo --- C Done ---
echo

#   C++
echo --- Starting C++ ---
g++ benchmarks/$folder/cpp/bench.cpp -O3 -o benchmarks/$folder/cpp/bench -L./target/release -lrapl_lib -Wl,-rpath=./target/release && ./benchmarks/$folder/cpp/bench $count $sleep_time
sleep 5s
bash utils/append_to_latest_csv.sh "Cpp$testName"
echo --- C++ Done ---
echo

#   Node
echo --- Starting JavaScript ---
node ./benchmarks/$folder/javascript/bench.js $count $sleep_time
sleep 5s
bash utils/append_to_latest_csv.sh "Node$testName"
echo --- JavaScript Done ---
echo

#   Python
echo --- Starting Python ---
python3 ./benchmarks/$folder/python/bench.py $count $sleep_time
sleep 5s
bash utils/append_to_latest_csv.sh "Python$testName"
echo --- Python Done ---
echo

#   Pypy
echo --- Starting PyPy ---
pypy ./benchmarks/$folder/python/bench.py $count $sleep_time
sleep 5s
bash utils/append_to_latest_csv.sh "Pypy$testName"
echo --- PyPy Done ---
echo

#   C#
echo --- Starting C# ---
dotnet run --project ./benchmarks/$folder/csharp/Sleep.csproj --configuration Release $count $sleep_time
sleep 5s
bash utils/append_to_latest_csv.sh "Csharp$testName"
echo --- C# Done ---
echo

#   Java
echo --- Starting Java ---
java --enable-native-access=ALL-UNNAMED --enable-preview --source 21 ./benchmarks/$folder/java/Bench.java $count $sleep_time
sleep 5s
bash utils/append_to_latest_csv.sh "Java$testName"
echo --- Java Done ---
echo

echo "!!! Finished $testName !!!"
