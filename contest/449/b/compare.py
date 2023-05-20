import subprocess
from datetime import datetime


def run_cpp_binary(binary_path, input_str):
    input_bytes = input_str.encode("utf-8")
    # Run the C++ binary and capture the output
    result = subprocess.run(binary_path, input=input_bytes, stdout=subprocess.PIPE, stderr=subprocess.PIPE)

    # Return the output as a string
    return result.stdout.decode('utf-8')


# Paths to the C++ binary files
binary1_path = "./main"
binary2_path = "./brute"
import random
random.seed(datetime.now())

def generate_numbers():
    output_str = ""

    # Generate a random number n from 1 to 10
    n = random.randint(2, 100)
    m = random.randint(1, 3000)
    k = random.randint(1, 100)

    output_str += f'{n} {m} {k}\n'

    list = range(1, n + 1)
    for i in range(m):
        sample = random.sample(list, 2)
        cost = random.randint(500000000, 1000000000)
        output_str += f'{sample[0]} {sample[1]} {cost}\n'

    for i in range(k):
        node = random.randint(2, n)
        cost = random.randint(500000000, 1000000000)
        output_str += f'{node} {cost}\n'

    return output_str


def run_single():
    input_str = generate_numbers()

    output = run_cpp_binary(binary1_path, input_str)
    if output == "":
        print(input_str)
        print("error")
        exit(0)


def run_testcase():
    input_str = generate_numbers()

    print("*** input ***\n" + input_str + "\n")
    # Run the binary files and get their output
    output1 = run_cpp_binary(binary1_path, input_str)
    output2 = run_cpp_binary(binary2_path, input_str)

    print('*************')
    print(output1)
    print('*************')
    print(output2)
    # Compare the output
    if output1 == output2:
        print("The output is the same!")
    else:
        print("The output is different.")
        exit(0)


while True:
    run_single()
# while True:
#     run_testcase()

