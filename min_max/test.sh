#!/bin/bash
for i in  {1..10}
do
    if target/debug/min_max test/input$i.txt | cmp -s - test/output$i.txt; then
        echo test-$i passed
    else
        echo test-$i error
    fi
done
