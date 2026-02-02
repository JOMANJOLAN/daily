package org.innuendo.leetcode.medium.design;

import java.util.HashMap;
import java.util.Map;
import java.util.Random;

/**
 * <h1>常数时间插入、删除和获取随机元素</h1>
 * Your RandomizedSet object will be instantiated and called as such:<br>
 * <code>
 * RandomizedSet obj = new RandomizedSet();<br>
 * boolean param_1 = obj.insert(val);<br>
 * boolean param_2 = obj.remove(val);<br>
 * int param_3 = obj.getRandom();<br>
 * </code>
 */
public class RandomizedSet {
    private static class LList {
        Random random = new Random();
        int[] data = new int[20000];
        int p = -1;

        public void reset() {
            p = -1;
        }

        public int getRandom() {
            int i = random.nextInt(p + 1);
            return data[i];
        }

        public int insert(int value) {
            p++;
            data[p] = value;
            return p;
        }

        public int remove(int i) {
            data[i] = data[p];
            p--;
            return data[i];
        }
    }

    private static final LList LIST = new LList();
    private final Map<Integer, Integer> map = new HashMap<>();

    public RandomizedSet() {
        LIST.reset();
    }

    public boolean insert(int val) {
        if (map.containsKey(val)) {
            return false;
        }
        int index = LIST.insert(val);
        map.put(val, index);
        return true;
    }

    public boolean remove(int val) {
        Integer index = map.get(val);
        if (null == index) {
            return false;
        }
        map.remove(val);
        int lastValue = LIST.remove(index);
        // `lastValue`的位置，变为了`index`
        if (lastValue != val) {
            map.put(lastValue, index);
        }
        return true;
    }

    public int getRandom() {
        return LIST.getRandom();
    }
}
