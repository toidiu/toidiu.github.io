/* ==========================================================================
   jQuery plugin settings and other scripts
   ========================================================================== */

$(document).ready(function(){
  $('.more').click(function() {
    event.preventDefault();
    $( this ).parent().toggleClass("hide-more")
  })
  $('.less').click(function() {
    event.preventDefault();
    $( this ).parent().toggleClass("hide-more")
  })

});
