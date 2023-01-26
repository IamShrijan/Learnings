using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Diagnostics;
using System.Collections;

public Class BinarySearchTrees 
{
    public class Node
    {
        public int Data;
        public Node Left;
        public Node Right;
        public Node(int Value )
        {
            Data = Value;
        }
        public void DisplayNode()
        {
            Console.Writeln(Data+"");
        }
    }

    public Node root;

    public BinarySearchTrees()
    {
        root = Null;
    }

    public void Insert(i)
    {
        Node newNode = new Node(i);
        if (root == Null)
        {
            root = newNode;
        }
        else
        {
            Node currentNode = root;
            Node parent;
            while (currentNode != Null)
            {
                parent = currentNode;
                if (i >= currentNode)
                {
                    currentNode = currentNode.Right;
                    
                    if (currentNode == Null)
                    {
                        parent.Left = newNode;
                    }
                }
                else 
                {
                    currentNode = currentNode.Left;
                    
                    if (currentNode == Null)
                    {
                        parent.Right = newNode;
                    }
                }
            }

        }

    }
    static void Main()
        {
            BinarySearchTree nums = new BinarySearchTree();
            nums.Insert(50);
            nums.Insert(17);
            nums.Insert(23);
            nums.Insert(12);
            nums.Insert(19);
            nums.Insert(54);
            nums.Insert(9);
            nums.Insert(14);
            nums.Insert(67);
            nums.Insert(76);
            nums.Insert(72);
        }
}
