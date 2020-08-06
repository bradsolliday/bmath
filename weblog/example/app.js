
var main = function() {
    "use strict";

    $(".comment-input input").on("keypress", function (event) {
        if (event.keyCode === 13) {
            appendComment();
        }
    });

    $(".comment-input button").on("click", appendComment);
};

var appendComment = function() {
    var comment_text = $(".comment-input input").val();

    if (comment_text !== "") {
        $(".comment-input input").val("");
        var $new_comment = $("<p>").text(comment_text);
        $new_comment.hide();
        $(".comments").append($new_comment);
        $new_comment.fadeIn();
    }
};

$(document).ready(main);