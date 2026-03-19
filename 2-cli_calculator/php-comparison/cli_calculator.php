<?php
function get_number(string $prompt): float {
    while (true) {
        echo $prompt;
        fflush(STDOUT);
        $input = fgets(STDIN);
        if ($input === false) {
            echo "Failed to read input.\n";
            continue;
        }
        $trimmed = trim($input);
        if (is_numeric($trimmed)) {
            return (float) $trimmed;
        }
        echo "Invalid number. Please enter a valid numeric value.\n";
    }
}

echo "Welcome to the PHP CLI Calculator!\n";
$num1 = get_number("Enter the first number: ");
$num2 = get_number("Enter the second number: ");
echo "Enter an operation (+, -, *, /): ";
fflush(STDOUT);
$operation = trim(fgets(STDIN) ?: '');

$result = null;
switch ($operation) {
    case '+': $result = $num1 + $num2; break;
    case '-': $result = $num1 - $num2; break;
    case '*': $result = $num1 * $num2; break;
    case '/':
        if ($num2 != 0) {
            $result = $num1 / $num2;
        } else {
            echo "Error: Division by zero is not allowed.\n";
        }
        break;
    default:
        echo "Invalid operation. Please enter +, -, *, or /.\n";
}

if ($result !== null) {
    echo "Result: $result\n";
}
