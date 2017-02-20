(function() {
    "use strict";

    angular
        .module('superstars.editor')
        .controller('EditorCtrl', EditorCtrl);

    EditorCtrl.$inject = ['$state'];

    function EditorCtrl($state) {
        var vm = this;

        vm.slug = $state.params.slug;
    }
})();