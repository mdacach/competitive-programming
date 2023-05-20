import subprocess


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


def generate_numbers():
    # Generate a random number n from 1 to 10
    n = random.randint(1, 11)

    # Generate n random numbers from 1 to 20
    numbers = [random.randint(1, 21) for _ in range(n)]

    # Prepend n to the beginning of the list
    numbers.insert(0, n)

    return numbers


import networkx as nx


def generate_random_tree(n):
    # Create an empty graph
    tree = nx.Graph()

    # Add the first vertex
    tree.add_node(1)

    # Add the remaining vertices one by one
    for i in range(2, n + 1):
        # Choose a random vertex that is already in the tree
        random_vertex = random.choice(list(tree.nodes()))

        # Add the new vertex and the edge connecting it to the random vertex
        tree.add_node(i)
        tree.add_edge(i, random_vertex)

    # Return the tree
    return tree


# Example usage
n = 10
tree = generate_random_tree(n)

# Print the edges of the tree
for v1, v2 in tree.edges():
    print(v1, v2)


def generate_numbers():
    output_str = ""

    # Generate a random number n from 1 to 10
    n = random.randint(1, 11)
    m = random.randint(1, n)
    d = random.randint(1, 20)
    output_str += f'{n} {m} {d}\n'

    list = range(1, n + 1)
    sample = random.sample(list, m)
    for v in sample:
        output_str += f'{v} '
    output_str += "\n"

    tree = generate_random_tree(n)
    for v1, v2 in tree.edges():
        output_str += f'{v1} {v2}\n'

    return output_str


def run_testcase():
    input_str = generate_numbers()

    print("input\n" + input_str + "\n")
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
    run_testcase()
