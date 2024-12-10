using System.Text;

namespace program;

public readonly record struct Point(int x, int y) {
    public bool inBounds(uint width, uint height) {
        return
            x >= 0 && x < width
            && y >= 0 && y < width;
    }
}

public class Solver {
    public Dictionary<char, List<Point>> antennasDict;
    public uint width;
    public uint height;

    public Solver() {
        String[] lines = File.ReadAllLines("input.txt");
        width = (uint) lines[0].Length;
        height = (uint) lines.Length;

        antennasDict =
            new Dictionary<char, List<Point>>();

        for (int y = 0; y < height; y++) {
            for (int x = 0; x < width; x++) {
                char c = lines[y][x];

                if (c == '.') continue;

                antennasDict.TryAdd(c, new List<Point>());
                antennasDict[c].Add(new Point(x, y));
            }
        }
    }

    static IEnumerable<(T, T)> Pairs<T>(IEnumerable<T> collection) {
        return
            from a in collection
            from b in collection
            where !a.Equals(b)
            select (a, b);
    }

    virtual public IEnumerable<Point> FindAntiNodes(Point pointA, Point pointB) {
        int dx = pointB.x - pointA.x;
        int dy = pointB.y - pointA.y;

        return new Point[] {
            new Point(pointA.x - dx, pointA.y - dy),
            new Point(pointB.x + dx, pointB.y + dy),
        };
    }

    public IEnumerable<Point> FindAllAntiNodes() {
        return (
            from keyValue in antennasDict
            let freq = keyValue.Key
            let antennas = keyValue.Value

            from pair in Pairs(antennas)
            let antiNodes = FindAntiNodes(pair.Item1, pair.Item2)
            from antiNode in antiNodes
            where antiNode.inBounds(width, height)
            select antiNode
        ).Distinct();
    }

    public void PrintAntiNodes(IEnumerable<Point> antiNodes) {
        StringBuilder output = new StringBuilder();
        for (int y = 0; y < height; y++) {
            for (int x = 0; x < width; x++) {
                output.Append('.');
            }
            output.Append('\n');
        }
        foreach (var antiNode in antiNodes) {
            output[(int)antiNode.y * ((int)width + 1) + antiNode.x] = '#';
        }
        Console.WriteLine(output.ToString());
    }
}

public class SolverPart2 : Solver {
    override public IEnumerable<Point> FindAntiNodes(Point pointA, Point pointB) {
        int dx = pointB.x - pointA.x;
        int dy = pointB.y - pointA.y;

        int countX = (int)width / int.Abs(dx);
        int countY = (int)height / int.Abs(dy);
        int count = int.Max(countX, countY);
        count = 100;

        return
            from i in Enumerable.Range(-count, count * 2)
            let dxi = dx * i
            let dyi = dy * i
            select new Point(pointA.x + dxi, pointA.y + dyi);
    }
}

public class Program {
    public static void PrintCollectionBy<T>(
        IEnumerable<T> collection,
        Func<T, String?> toString
    ) {
        foreach (T item in collection) {
            Console.WriteLine(toString(item));
        }

        Console.WriteLine("{0}\n", collection.Count());
    }

    public static void PrintCollection<T>(IEnumerable<T> collection) {
        PrintCollectionBy(collection, item => item?.ToString());
    }

    public static void Main() {
        Solver solver = new Solver();

        var antiNodes = solver.FindAllAntiNodes();
        PrintCollection(antiNodes);

        SolverPart2 solver2 = new SolverPart2();

        var antiNodes2 = solver2.FindAllAntiNodes();
        PrintCollection(antiNodes2);

        solver2.PrintAntiNodes(antiNodes2);
    }
}
