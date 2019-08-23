port module Main exposing (Model, Msg(..), ensureTrailingNewline, init, main, update, view)

import Browser
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onClick, onInput)
import Html.Parser
import Html.Parser.Util exposing (toVirtualDom)
import Http exposing (Error(..))
import Json.Decode as Decode
import Json.Encode as Enc
import Parser exposing (deadEndsToString)
import String exposing (endsWith)



-- ---------------------------
-- MODEL
-- ---------------------------


{-| The entire app's state. Similar to the Store in React/Redux.
-}
type alias Model =
    { plaintextScreenplay : String -- Plain text the user types in, encoded in Fountain markup
    , renderedScreenplay : String -- The styled text, generated from the plaintext
    , serverMessage : String -- Error messages if the user's markup was invalid
    , viewMode : ViewMode
    }


type ViewMode
    = Dual
    | Write
    | Print


{-| What should the Model be when the user starts the app?
-}
init : String -> ( Model, Cmd Msg )
init flags =
    ( { plaintextScreenplay = flags
      , serverMessage = ""
      , renderedScreenplay = ""
      , viewMode = Dual
      }
    , postScreenplay flags
    )



-- ---------------------------
-- UPDATE
-- ---------------------------


{-| Union/enum/ADT of every event that could happen in the app.
-}
type Msg
    = ChangeScreenplay String -- User edited their plaintext screenplay
    | RenderBtnPress -- User pressed the Render button
    | SetViewMode ViewMode -- Change which View Mode to render the UI in
    | RenderResponse (Result Http.Error String) -- The backend returned with rendered screenplay


{-| Given some Msg, and the current Model, output the new model and a side-effect to execute.
-}
update : Msg -> Model -> ( Model, Cmd Msg )
update message model =
    case message of
        ChangeScreenplay s ->
            ( { model | plaintextScreenplay = s }, Cmd.none )

        RenderBtnPress ->
            ( model, postScreenplay model.plaintextScreenplay )

        SetViewMode vm ->
            ( { model | viewMode = vm }, postScreenplay model.plaintextScreenplay )

        RenderResponse res ->
            case res of
                Ok r ->
                    ( { model | renderedScreenplay = r }, Cmd.none )

                Err err ->
                    ( { model | serverMessage = "Error: " ++ httpErrorToString err }, Cmd.none )


httpErrorToString : Http.Error -> String
httpErrorToString err =
    case err of
        BadUrl _ ->
            "BadUrl"

        Timeout ->
            "Timeout"

        NetworkError ->
            "NetworkError"

        BadStatus _ ->
            "BadStatus"

        BadBody s ->
            "BadBody: " ++ s



-- ---------------------------
-- HTTP
-- ---------------------------


{-| Send HTTP request to the Fountain backend. Request contains the plaintext screenplay,
response will contain the rendered screenplay.
-}
postScreenplay : String -> Cmd Msg
postScreenplay s =
    Http.post
        { url = "https://screenplay.page/renderfountain"
        , body =
            Http.jsonBody <|
                Enc.object
                    [ ( "screenplay", Enc.string <| ensureTrailingNewline s )
                    ]
        , expect = Http.expectString RenderResponse
        }


ensureTrailingNewline s =
    if endsWith "\n" s then
        s

    else
        s ++ "\n"



-- ---------------------------
-- VIEW
-- ---------------------------


view : Model -> Html Msg
view model =
    case model.viewMode of
        Print ->
            printViewMode model

        Dual ->
            dualViewMode model

        Write ->
            writeViewMode model


writeViewMode : Model -> Html Msg
writeViewMode model =
    div [ class "container-write-pane" ]
        [ pageHeader model
        , div [ class "editor editor-in" ] [ userTextInput model ]
        ]


printViewMode : Model -> Html Msg
printViewMode model =
    div [ class "container-print-pane" ]
        [ pageHeader model
        , div [ class "editor editor-out" ] (outputPane model)
        ]


dualViewMode : Model -> Html Msg
dualViewMode model =
    div [ class "container-two-pane" ]
        [ pageHeader model
        , div [ class "editor editor-in" ]
            [ userTextInput model
            , br [] []
            ]
        , div [ class "editor editor-out" ]
            (outputPane model)
        , footerDiv
        ]


pageHeader model =
    let
        maybeBtn =
            case model.viewMode of
                Write ->
                    []

                Dual ->
                    [ renderBtn ]

                Print ->
                    []

        buttons =
            maybeBtn
                ++ [ viewModeBtn model Dual
                   , viewModeBtn model Print
                   , viewModeBtn model Write
                   ]
    in
    header []
        [ h1 [] [ text "Screenplay Editor" ]
        , div [] buttons
        ]


viewModeBtn : Model -> ViewMode -> Html Msg
viewModeBtn model viewMode =
    let
        buttonClass =
            if model.viewMode == viewMode then
                "pure-button-selected"

            else
                "pure-button"
    in
    button
        [ class buttonClass, onClick (SetViewMode viewMode) ]
        [ text <| toString viewMode ]


toString : ViewMode -> String
toString vm =
    case vm of
        Write ->
            "Write View"

        Print ->
            "Print View"

        Dual ->
            "Two-Panel View"


footerDiv =
    footer []
        [ p []
            [ text "Made by "
            , link "https://twitter.com/adam_chal" "@adam_chal"
            , text ". Parsing done in Rust via my "
            , link "https://crates.io/crates/fountain" "Fountain"
            , text " crate, which is compiled into WebAssembly and run in the browser via "
            , link "https://blog.cloudflare.com/introducing-wrangler-cli/" "Cloudflare Workers"
            , text ". Frontend written in Elm. Functionality also available via "
            , link "https://github.com/adamchalmers/fountain-rs" "CLI"
            , text ". Want to save a PDF? Switch to Print View then use your in-browser print to save as PDF."
            ]
        ]


{-| Convenience function for simpler <a> links
-}
link to txt =
    a [ href to, target "_blank" ] [ text txt ]


{-| When users click this button, the backend will style their screenplay
-}
renderBtn =
    button
        [ class "pure-button pure-button-primary", onClick RenderBtnPress ]
        [ text "Render screenplay" ]


{-| This is where users type their plaintext screenplays
-}
userTextInput model =
    textarea
        [ onInput ChangeScreenplay
        , rows 20
        , cols 40
        ]
        [ text model.plaintextScreenplay ]


{-| This is where users see their rendered screenplay
-}
outputPane model =
    if model.serverMessage == "" then
        case Html.Parser.run model.renderedScreenplay of
            Ok html ->
                toVirtualDom html

            Err errs ->
                [ text <| deadEndsToString errs ]

    else
        [ text <| model.serverMessage ]



-- ---------------------------
-- MAIN
-- ---------------------------


{-| Wire all the various components together
-}
main : Program String Model Msg
main =
    Browser.document
        { init = init
        , update = update
        , view =
            \m ->
                { title = "Write a screenplay in Elm"
                , body = [ view m ]
                }
        , subscriptions = \_ -> Sub.none
        }
