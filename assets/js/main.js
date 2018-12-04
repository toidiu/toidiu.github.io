/* ==========================================================================
   jQuery plugin settings and other scripts
   ========================================================================== */

$(document).ready(function(){
  $('.more').click(function() {
    $( this ).parent().toggleClass("hide-more")
  })
  $('.less').click(function() {
    $( this ).parent().toggleClass("hide-more")
  })

});
