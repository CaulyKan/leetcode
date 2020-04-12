using System;
using System.Collections.Generic;


public class LFUCache
{
    public LFUCache(int cap)
    {
        this.Capacity = cap;
        this.FirstRoom = new LFURoom { Freq = 0 };
        this.LastRoom = this.FirstRoom;
    }

    public int Capacity { get; set; }
    public int Length { get; set; }
    public LFUNode First { get; set; }
    public LFUNode Last { get; set; }
    public LFURoom FirstRoom { get; set; }
    public LFURoom LastRoom { get; set; }
    public Dictionary<int, LFUNode> Map { get; set; } = new Dictionary<int, LFUNode>();

    public override string ToString()
    {
        var current = this.FirstRoom;
        var result = "";
        do
        {
            result += ($"[f{current.Freq}: {current}] ");
            current = current.Next;
        } while (current != null);
        result += "\n";

        var currentNode = this.First;
        while (currentNode != null)
        {
            result += $"{currentNode.Key},";
            if (currentNode.Next != null && currentNode.Next.Previous != currentNode)
            {
                throw new ArgumentException();
            }
            currentNode = currentNode.Next;
        }
        return result;
    }

    public int Get(int key)
    {
        LFUNode node;
        if (this.Map.TryGetValue(key, out node))
        {
            this.IncreaseFreq(node);
            return node.Val;
        }
        else
        {
            return -1;
        }
    }

    public void Put(int key, int val)
    {
        if (this.Capacity > 0)
        {
            if (!this.Map.ContainsKey(key))
            {
                var node = new LFUNode { Val = val, Key = key };
                this.Length += 1;
                if (this.Length > this.Capacity)
                {
                    this.Map.Remove(this.First.Room.LastNode.Key);
                    this.RemoveNode(this.First.Room.LastNode);
                    this.Length -= 1;
                }
                this.Map.Add(key, node);
                this.InsertNode(node, this.First, this.FirstRoom);
            }
            else
            {
                this.Map[key].Val = val;
                this.IncreaseFreq(this.Map[key]);
            }
        }
    }

    public void IncreaseFreq(LFUNode node)
    {
        if (node.Room.Next == null)
        {
            node.Room.Next = new LFURoom() { Freq = node.Room.Freq + 1 };
        }
        var newRoom = node.Room.Next;
        var next = node.Room.LastNode?.Next;
        this.RemoveNode(node);
        this.InsertNode(node, next, newRoom);
    }

    public void InsertNode(LFUNode node, LFUNode next, LFURoom room)
    {
        if (next != null)
        {
            if (next.Previous != null)
            {
                var prev = next.Previous;
                prev.Next = node;
                next.Previous = node;
                node.Previous = prev;
                node.Next = next;
            }
            else
            {
                node.Next = next;
                next.Previous = node;
                this.First = node;
            }
        }
        else
        {
            if (this.Last != null)
            {
                this.Last.Next = node;
                node.Previous = this.Last;
            }
            this.Last = node;
            if (this.First == null)
                this.First = node;
            node.Next = null;
        }

        if (room.FirstNode == null)
        {
            room.FirstNode = node;
            room.LastNode = node;
        }
        if (room.FirstNode == next)
        {
            room.FirstNode = node;
        }
        node.Room = room;
    }

    public void RemoveNode(LFUNode node)
    {
        if (node.Next != null)
        {
            if (node.Previous != null)
            {
                node.Previous.Next = node.Next;
                node.Next.Previous = node.Previous;
            }
            else
            {
                node.Next.Previous = null;
                this.First = node.Next;
            }

            if (node.Room.FirstNode == node)
            {
                if (node.Room == node.Next.Room)
                    node.Room.FirstNode = node.Next;
                else node.Room.FirstNode = null;
            }

            if (node.Room.LastNode == node)
            {
                if (node.Room == node.Previous?.Room)
                {
                    node.Room.LastNode = node.Previous;
                }
                else node.Room.LastNode = null;
            }
        }
        else
        {
            if (node.Previous != null)
            {
                node.Previous.Next = null;
            }

            if (node.Room.FirstNode == node)
            {
                node.Room.FirstNode = null;
            }
            if (node.Room.LastNode == node)
            {
                if (node.Previous?.Room == node.Room)
                {
                    node.Room.LastNode = node.Previous;
                }
                else node.Room.LastNode = null;
            }
            this.Last = node.Previous;
        }
    }
}

public class LFUNode
{
    public int Val { get; set; }
    public LFURoom Room { get; set; }
    public LFUNode Previous { get; set; }
    public LFUNode Next { get; set; }
    public int Key { get; set; }
}

public class LFURoom
{
    public int Freq { get; set; }
    public LFUNode FirstNode { get; set; }
    public LFUNode LastNode { get; set; }
    public LFURoom Next { get; set; }

    public override string ToString()
    {
        var current = this.FirstNode;
        var result = "";
        while (current != null && current.Room == this)
        {
            result += $"{current.Key},";
            current = current.Next;
        }

        result += $"!{this.LastNode?.Key}";
        return result;
    }
}

class Program
{
    static void Main(string[] args)
    {
        var lfu = new LFUCache(10);
        var actions = "put,put,put,put,put,get,put,get,get,put,get,put,put,put,get,put,get,get,get,get,put,put,get,get,get,put,put,get,put,get,put,get,get,get,put,put,put,get,put,get,get,put,put,get,put,put,put,put,get,put,put,get,put,put,get,put,put,put,put,put,get,put,put,get,put,get,get,get,put,get,get,put,put,put,put,get,put,put,put,put,get,get,get,put,put,put,get,put,put,put,get,put,put,put,get,get,get,put,put,put,put,get,put,put,put,put,put,put,put".Split(",");
        var vals = "10,13],[3,17],[6,11],[10,5],[9,10],[13],[2,19],[2],[3],[5,25],[8],[9,22],[5,5],[1,30],[11],[9,12],[7],[5],[8],[9],[4,30],[9,3],[9],[10],[10],[6,14],[3,1],[3],[10,11],[8],[2,14],[1],[5],[4],[11,4],[12,24],[5,18],[13],[7,23],[8],[12],[3,27],[2,12],[5],[2,9],[13,4],[8,18],[1,7],[6],[9,29],[8,21],[5],[6,30],[1,12],[10],[4,15],[7,22],[11,26],[8,17],[9,29],[5],[3,4],[11,30],[12],[4,29],[3],[9],[6],[3,4],[1],[10],[3,29],[10,28],[1,20],[11,13],[3],[3,12],[3,8],[10,9],[3,26],[8],[7],[5],[13,17],[2,27],[11,15],[12],[9,19],[2,15],[3,16],[1],[12,17],[9,1],[6,19],[4],[5],[5],[8,1],[11,7],[5,2],[9,28],[1],[2,2],[7,4],[4,22],[7,24],[9,26],[13,28],[11,26".Split("],[");
        for (int i = 0; i < actions.Length; i++)
        {
            if (actions[i] == "put")
            {
                var kv = vals[i].Split(",");
                lfu.Put(int.Parse(kv[0]), int.Parse(kv[1]));
                Console.WriteLine($"Put {vals[i]}");
            }
            else
            {
                var result = lfu.Get(int.Parse(vals[i]));
                Console.WriteLine($"Get {vals[i]} -> {result}");
            }
            Console.WriteLine(lfu.ToString());
        }
    }
}
