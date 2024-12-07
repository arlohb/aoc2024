type Input = { Result: int64; Nums: int64 list }

let init list = List.take ((List.length list) - 1) list

let (|Last|_|) list =
    match list with
    | [] -> None
    | xs -> Some(init xs, List.last xs)

let rec resolve inputs ops =
    match (inputs, ops) with
    | ([], _) -> 0L
    | (Last(_, i), []) -> i
    | (Last(inputs, i), Last(ops, op)) -> op (resolve inputs ops) i
    | (_, _) -> 0L

let rec genCombinations xs n =
    match n with
    | 1 -> Seq.map Seq.singleton xs
    | n ->
        genCombinations xs (n - 1)
        |> Seq.collect (fun ops -> xs |> Seq.map (fun x -> Seq.append ops [ x ]))

let checkInput ops { Result = result; Nums = nums } =
    genCombinations ops (nums |> List.length |> (+) -1)
    |> Seq.map Seq.toList
    |> Seq.map (resolve nums)
    |> Seq.exists ((=) result)

let concatOp x y = x.ToString() + y.ToString() |> int64

[<EntryPoint>]
let main _ =
    let input =
        System.IO.File.ReadAllLines("input.txt")
        |> Array.map (fun s ->
            let result :: nums = s.Split(" ") |> Array.toList

            { Result = result |> String.filter System.Char.IsDigit |> int64
              Nums = nums |> List.map int64 })

    input
    |> Array.filter (checkInput [ (+); (*) ])
    |> Array.sumBy _.Result
    |> printfn "%d"

    input
    |> Array.filter (checkInput [ (+); (*); concatOp ])
    |> Array.sumBy _.Result
    |> printfn "%d"

    0
