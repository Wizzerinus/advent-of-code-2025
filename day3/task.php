<?php
function get_max($line, $size) {
    if (!$line) return 0;
    $items = [];
    for ($i = 0; $i < $size; $i++) array_push($items, $i);
    for ($i = 0; $i < strlen($line); $i++) {
        $sym = (int) $line[$i];
        for ($j = 0; $j < $size; $j++) {
            if ($items[$j] < $i && $i + $size - $j - 1 < strlen($line) && (int) $line[$items[$j]] < $sym) {
                array_splice($items, $j);
                for ($k = 0; $k + $j < $size; $k++) {
                    array_push($items, $i + $k);
                }
                break;
            }
        }
    }
    $out = 0;
    foreach ($items as $it) {
        $out = ($out * 10) + (int) $line[$it];
    }
    return $out;
}

$text = file_get_contents("./input3.txt");
$lines = explode("\n", $text);
$task1 = 0;
$task2 = 0;
foreach ($lines as $line) {
    $task1 += get_max($line, 2);
    $task2 += get_max($line, 12);
}

?>
First task: <?= $task1 ?>

Second task: <?= $task2 ?>

