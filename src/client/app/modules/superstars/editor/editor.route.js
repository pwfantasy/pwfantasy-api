(function() {
    "use strict";

    angular
        .module('superstars.editor')
        .config(configuration);

    function configuration($stateProvider) {
        $stateProvider
            .state('superstars.editor', {
                url: '/edit/:slug',
                templateUrl: '/app/modules/superstars/editor/editor.view.html',
                controller: 'EditorCtrl',
                controllerAs: 'editorVm'
            });
    }
})();