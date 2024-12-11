
let read_file file =
    In_channel.with_open_text file In_channel.input_all ;;

module Point = struct
    type point = Point of int * int

    let print (Point (x,y)) = Printf.printf "Point (%d, %d)" x y
    let add (Point (x1,y1)) (Point (x2,y2)) = Point (x1+x2, y1+y2)
end;;

let get_2d array (Point.Point (x,y)) = array.(y).(x) ;;

let find_starts array = array
    |> Array.to_list
    |> List.mapi (fun yi cs -> cs
        |> Array.to_list
        |> List.mapi (fun i c -> i, c)
        |> List.filter_map (fun (xi, c) ->
            match c with
            | 0 -> Some (Point.Point (xi, yi))
            | _ -> None
        )
    )
    |> List.flatten
;;

let directions = [|
    Point.Point (0,1);
    Point.Point (1,0);
    Point.Point (0,-1);
    Point.Point (-1,0);
|] ;;

let width map = map.(0) |> Array.length ;;
let height = Array.length

let in_bounds map (Point.Point (x,y)) =
    x >= 0
    && x < (width map)
    && y >= 0
    && y < (height map)
;;

let next_points map point n =
    directions
    |> Array.map (Point.add point)
    |> Array.to_list
    |> List.filter (in_bounds map)
    |> List.filter (fun point -> get_2d map point == n + 1)
;;

let sum = List.fold_left (+) 0

let rec all_paths' map point n =
    match n with
    | 9 -> [ point ]
    | n -> next_points map point n
        |> List.map(fun p -> all_paths' map p (n + 1))
        |> List.flatten
;;

let rec distinct =
    function
    | [] -> []
    | x :: xs -> x :: distinct (xs |> List.filter ((<>) x))

let all_paths map start =
    all_paths' map start 0
    |> distinct
;;

let (>>) x y a = y (x a) ;;

let main () =
    let input = read_file "input.txt" in
    let lines = String.split_on_char '\n' input in

    let map = lines
        |> List.map (fun line -> line
            |> String.to_seq
            |> Array.of_seq
            |> (Array.map (fun c -> (Char.code c) - 48))
        )
        |> List.filter (fun arr -> (Array.length arr) != 0)
        |> Array.of_list
    in

    find_starts map
        |> List.map (fun point -> all_paths map point)
        |> List.map List.length
        |> sum
        |> (print_int >> print_newline);
;;

main ()

