import subprocess
import random
import networkx as nx  # For graph generation


def run_binary(binary_path, input_str):
    input_bytes = input_str.encode("utf-8")

    # Run the binary and capture the output
    result = subprocess.run(binary_path, input=input_bytes, stdout=subprocess.PIPE, stderr=subprocess.PIPE)

    # Return the output
    return result


def generate_random_tree(n):
    # Create an empty graph
    graph = nx.Graph()

    # Add the first vertex
    graph.add_node(1)

    # Add the remaining vertices one by one
    for i in range(2, n + 1):
        # Choose a random vertex that is already in the graph
        random_vertex = random.choice(list(graph.nodes()))

        # Add the new vertex and the edge connecting it to the random vertex
        graph.add_node(i)
        graph.add_edge(i, random_vertex)

    # Return the tree
    return graph


def generate_input():
    output_str = ""

    # Generate a random number n from 1 to 10
    n = random.randint(1, 11)
    m = random.randint(1, n)
    d = random.randint(1, 20)
    output_str += f'{n} {m} {d}\n'

    possible = range(1, n + 1)
    sample = random.sample(possible, m)
    for v in sample:
        output_str += f'{v} '
    output_str += "\n"

    tree = generate_random_tree(n)
    for v1, v2 in tree.edges():
        output_str += f'{v1} {v2}\n'

    return output_str


def run_testcase(main_path, brute_path, debug=False):
    input_str = generate_input()

    if debug:
        print("**** input ****\n" + input_str + "*************\n")

    # Run the binary files and get their output
    main_output = run_binary(main_path, input_str)
    brute_output = run_binary(brute_path, input_str)

    if main_output.returncode != 0:
        print(f"Main returned code {main_output.returncode}")
        print("Input saved to 'temp_input.txt'")
        with open('temp_input.txt', 'w') as file:
            file.write(input_str)
        exit(0)

    main_output = main_output.stdout.decode("utf8")
    brute_output = brute_output.stdout.decode("utf8")

    if debug:
        print('**** output main ****')
        print(main_output)
        print('*************')
        print('**** output brute ****')
        print(brute_output)

    # Compare the output
    if main_output == brute_output:
        if debug:
            print("The output is the same!")
    else:
        print("The output differs.")
        print("Input saved to 'temp_input.txt'")
        with open('temp_input.txt', 'w') as file:
            file.write(input_str)
        exit(0)


while True:
    run_testcase(main_path="./main", brute_path="./brute", debug=False)
