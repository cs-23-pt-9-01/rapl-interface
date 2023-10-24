﻿using System;
using System.Runtime.InteropServices;
using System.Numerics;

// inspired from https://stackoverflow.com/questions/24374658/check-the-operating-system-at-compile-time 
#if _LINUX
    const string pathToLib = @"target/release/librapl_lib.so";
#elif _WINDOWS
    const string pathToLib = @"target\release\rapl_lib.dll";
#else
    const string pathToLib = "none";
#endif

string[] arguments = Environment.GetCommandLineArgs();
uint count = uint.Parse(arguments[2]);
uint fibVal = uint.Parse(arguments[1]);

// DLL imports
[DllImport(pathToLib)]
static extern int start_rapl();

[DllImport(pathToLib)]
static extern void stop_rapl();

// test method
static ulong Fib(uint x)
{
    if (x == 0) return 0;

    ulong prev = 0;
    ulong next = 1;
    for (int i = 1; i < x; i++)
    {
        ulong sum = prev + next;
        prev = next;
        next = sum;
    }
    return next;
}

// Modified test method that uses big integers
static BigInteger FibBig(uint x)
{
    if (x == 0) return 0;

    BigInteger prev = new BigInteger(0);
    BigInteger next = new BigInteger(1);
    for (uint i = 1; i < x; i++)
    {
        BigInteger sum = prev + next;
        prev = next;
        next = sum;
    }
    return next;
}

// running benchmark
for (int i = 0; i < count; i++)
{
    start_rapl();

    var result = FibBig(fibVal);

    stop_rapl();
    if (result < 42){
        Console.WriteLine(result.ToString());
    }
}
Console.WriteLine("C# job done");
