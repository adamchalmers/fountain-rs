module Example exposing (unitTest)

import Expect exposing (Expectation)
import Fuzz exposing (Fuzzer, int, list, string)
import Main exposing (..)
import Test exposing (..)
import Test.Html.Query as Query
import Test.Html.Selector exposing (tag, text)


{-| See <https://github.com/elm-community/elm-test>
-}
unitTest : Test
unitTest =
    describe "ensureTrailingNewline"
        [ test "string ends with newline" <|
            \() ->
                ensureTrailingNewline "adam\n"
                    |> Expect.equal "adam\n"
        , test "string without newline" <|
            \() ->
                ensureTrailingNewline "adam"
                    |> Expect.equal "adam\n"
        ]
