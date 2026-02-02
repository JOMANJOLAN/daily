package org.innuendo.leetcode.medium.design;

import java.util.Arrays;

/**
 * <h1>二叉树的序列化与反序列化</h1>
 * Your Codec object will be instantiated and called as such:<br>
 * <code>
 * Codec ser = new Codec();<br>
 * Codec deSer = new Codec();<br>
 * TreeNode ans = deSer.deserialize(ser.serialize(root));
 * </code>
 */

//public class Codec {
//    // 只适用于无重复元素的情况
//    private void serializePreorder(TreeNode root, StringBuilder sb) {
//        if (root == null) return;
//        sb.append(root.val).append(" ");
//        serializePreorder(root.left, sb);
//        serializePreorder(root.right, sb);
//    }
//
//    private void serializeInorder(TreeNode root, StringBuilder sb) {
//        if (root == null) return;
//        serializeInorder(root.left, sb);
//        sb.append(root.val).append(" ");
//        serializeInorder(root.right, sb);
//    }
//
//    // Encodes a tree to a single string.
//    public String serialize(TreeNode root) {
//        if (root == null) return "";
//        var sb = new StringBuilder();
//        serializePreorder(root, sb);
//        sb.deleteCharAt(sb.length() - 1);
//        sb.append("\n");
//        serializeInorder(root, sb);
//        sb.deleteCharAt(sb.length() - 1);
//        return sb.toString();
//    }
//
//    private int[] preorder = null;
//    private int[] inorder = null;
//
//    private TreeNode decode(int pStart, int iStart, int length) {
//        if (length == 0) return null;
//        var pivot = preorder[pStart];
//        var i = 0;
//        for (; i < length; i++) {
//            if (inorder[iStart + i] == pivot) {
//                break;
//            }
//        }
//        var node = new TreeNode(pivot);
//        node.left = decode(pStart + 1, iStart, i);
//        node.right = decode(pStart + 1 + i, iStart + i + 1, length - 1 - i);
//        return node;
//    }
//
//    // Decodes your encoded data to tree.
//    public TreeNode deserialize(String data) {
//        if (data.isEmpty()) return null;
//        var strings = data.split("\n");
//        preorder = Arrays.stream(strings[0].split(" ")).mapToInt(Integer::parseInt).toArray();
//        inorder = Arrays.stream(strings[1].split(" ")).mapToInt(Integer::parseInt).toArray();
//        var length = preorder.length;
//        return decode(0, 0, length);
//    }
//}

public class Codec {
    public static class TreeNode {
        int val;
        TreeNode left;
        TreeNode right;

        TreeNode(int x) {
            val = x;
        }
    }

    private static final String SEP = " ";
    private static final String NULL = "#";

    // Encodes a tree to a single string.
    public String serialize(TreeNode root) {
        StringBuilder sb = new StringBuilder();
        serialize(root, sb);
        return sb.toString();
    }

    private void serialize(TreeNode root, StringBuilder sb) {
        if (root == null) {
            sb.append(NULL).append(SEP);
            return;
        }
        sb.append(root.val).append(SEP);
        serialize(root.left, sb);
        serialize(root.right, sb);
    }

    private int index = 0;

    // Decodes your encoded data to tree.
    public TreeNode deserialize(String data) {
        String[] nodes = data.trim().split(SEP);
        index = 0;
        return deserialize(nodes);
    }

    private TreeNode deserialize(String[] nodes) {
        if (index >= nodes.length || nodes[index].equals(NULL)) {
            index++;
            return null;
        }
        TreeNode node = new TreeNode(Integer.parseInt(nodes[index++]));
        node.left = deserialize(nodes);
        node.right = deserialize(nodes);
        return node;
    }
}
