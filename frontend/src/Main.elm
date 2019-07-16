port module Main exposing (Model, Msg(..), init, main, toJs, update, view)

import Browser
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onClick, onInput)
import Html.Parser
import Html.Parser.Util exposing (toVirtualDom)
import Http exposing (Error(..))
import Json.Decode as Decode
import Json.Encode as Enc



-- ---------------------------
-- PORTS
-- ---------------------------


port toJs : String -> Cmd msg



-- ---------------------------
-- MODEL
-- ---------------------------


type alias Model =
    { screenplay : String
    , rendered : String
    , serverMessage : String
    }


init : String -> ( Model, Cmd Msg )
init flags =
    ( { screenplay = flags, serverMessage = "", rendered = "" }, Cmd.none )


sampleText =
    """INT. MESS

The entire crew is seated. Hungrily swallowing huge portions of artificial food. The cat eats from a dish on the table.

KANE
First thing I'm going to do when we get back is eat some decent food.
"""



-- ---------------------------
-- UPDATE
-- ---------------------------


type Msg
    = TestServer
    | OnServerResponse (Result Http.Error String)
    | ChangeScreenplay String
    | RenderBtnPress
    | RenderResponse (Result Http.Error String)


update : Msg -> Model -> ( Model, Cmd Msg )
update message model =
    case message of
        ChangeScreenplay s ->
            ( { model | screenplay = s }, Cmd.none )

        RenderBtnPress ->
            ( model, postScreenplay model.screenplay )

        TestServer ->
            let
                expect =
                    Http.expectJson OnServerResponse (Decode.field "result" Decode.string)
            in
            ( model
            , Http.get { url = "/test", expect = expect }
            )

        OnServerResponse res ->
            case res of
                Ok r ->
                    ( { model | serverMessage = r }, Cmd.none )

                Err err ->
                    ( { model | serverMessage = "Error: " ++ httpErrorToString err }, Cmd.none )

        RenderResponse res ->
            case res of
                Ok r ->
                    ( { model | rendered = r }, Cmd.none )

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


postScreenplay : String -> Cmd Msg
postScreenplay s =
    -- Send HTTP request to the tunnel HTTP API, expect response to just be a string
    Http.post
        { url = "https://adamchalmers.com/fountain"
        , body =
            Http.jsonBody <|
                Enc.object
                    [ ( "screenplay", Enc.string s )
                    ]
        , expect = Http.expectString RenderResponse
        }



-- ---------------------------
-- VIEW
-- ---------------------------


view : Model -> Html Msg
view model =
    div [ class "container" ]
        [ header []
            [ h1 [] [ text "Fountain-rs live demo" ]
            ]
        , div [ class "pure-g" ]
            [ div [ class "pure-u-1-2" ]
                [ textarea [ onInput ChangeScreenplay, cols 40, rows 20 ] [ text model.screenplay ]
                , button
                    [ class "pure-button pure-button-primary"
                    , onClick RenderBtnPress
                    ]
                    [ text "Render" ]
                ]
            , div [ class "pure-u-1-2" ] [ div [ style "border" "1px solid gray" ] <| rendersOf model ]
            ]
        , div [] [ a [ href "https://fountain.io/", target "_blank" ] [ text "New to Fountain?" ] ]
        ]


rendersOf model =
    case Html.Parser.run model.rendered of
        Ok html ->
            html |> toVirtualDom

        Err err ->
            [ text "Render error" ]



-- ---------------------------
-- MAIN
-- ---------------------------


main : Program String Model Msg
main =
    Browser.document
        { init = init
        , update = update
        , view =
            \m ->
                { title = "Elm 0.19 starter"
                , body = [ view m ]
                }
        , subscriptions = \_ -> Sub.none
        }
