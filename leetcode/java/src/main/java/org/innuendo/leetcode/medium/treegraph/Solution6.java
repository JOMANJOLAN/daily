package org.innuendo.leetcode.medium.treegraph;

import java.util.Stack;

/**
 * 岛屿数量
 */
public class Solution6 {
    public int numIslands(char[][] grid) {
        var num = 0;
        int m = grid.length, n = grid[0].length;
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (grid[i][j] == '1') {
                    num++;
                    var stack = new Stack<int[]>();
                    stack.push(new int[]{i, j});
                    while (!stack.isEmpty()) {
                        var pop = stack.pop();
                        int popI = pop[0], popJ = pop[1];
                        if (0 <= popI && popI < m
                                && 0 <= popJ && popJ < n) {
                            if (grid[popI][popJ] == '1') {
                                grid[popI][popJ] = '0';
                                stack.push(new int[]{popI + 1, popJ});
                                stack.push(new int[]{popI - 1, popJ});
                                stack.push(new int[]{popI, popJ + 1});
                                stack.push(new int[]{popI, popJ - 1});
                            }
                        }
                    }
                }
            }
        }
        return num;
    }
}
