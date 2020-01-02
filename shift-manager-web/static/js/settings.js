/*
  Copyright (C) 2019  Simone Martelli

  This program is free software: you can redistribute it and/or modify
  it under the terms of the GNU Affero General Public License as published by
  the Free Software Foundation, either version 3 of the License, or
  (at your option) any later version.

  This program is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  GNU Affero General Public License for more details.

  You should have received a copy of the GNU Affero General Public License
  along with this program. If not, see <https://www.gnu.org/licenses/>.

  Contact info:
  - email:  simone.martelli.98@gmail.com
*/

/**
*
*/
const SettingsData = {
  structure: null,
  expansion: null
};

$(document).ready(function() {
  API.loadShiftStructure().done(function(structure, message, e) {
    if(e.status == 200) {
      SettingsData.structure = structure;
      API.loadShiftExpansion(structure.id).done(function(expansion) {
        SettingsData.expansion = expansion;
        SettingsController.init();
      }).fail(function() {
        SettingsController.init();
      });
    } else {
      SettingsController.init();
    }
  }).fail(function() {
    SettingsController.init();
  });

});

/**
*
*/
const SettingsController = (function() {

  let shiftTable = {
    'table': null,
    'body': null
  };

  function initEmpty() {
    $('#datetimepicker').datetimepicker({
      locale: 'it',
      format: 'YYYY-MM-DD'
    });
  }

  function init() {
    if(SettingsData.structure != null) {
      $('#datetimepicker').datetimepicker({
        locale: 'it',
        format: 'YYYY-MM-DD',
        defaultDate: SettingsData.structure.day
      });
    } else {
      initEmpty();
    }
    initTable();
    initEvent();
  }

  function initTable() {
    shiftTable.table = $('#tableShift');
    shiftTable.body = $(shiftTable.table.find('tbody')[0]);
    if(SettingsData.expansion != null) {
      var tmpl = $.templates("#template");
      shiftTable.body.html(tmpl.render({data: SettingsData.expansion}));
    }
    initTableEvent();
  }

  function initTableEvent() {
    shiftTable.body.find('input').change(function() {
      let tr = $(this).parent().parent();
      saveExpansion(tr);
    });
  }

  function initEvent() {
    $('#datetimepicker').on('change.datetimepicker', function(date) {
      saveStructure(date);
    });
    if(SettingsData.structure != null) {
      $('#plus').click(function() {
        addExpansion();
      });
      $('#minus').click(function() {
        removeExpansion();
      });
    }
  }

  function saveStructure(date) {
    if(SettingsData.structure == null) {
      SettingsData.structure = {};
      SettingsData.structure.day = date.date.format('YYYY-MM-DD');
      API.insertShiftStructure(SettingsData.structure).done(function() {
        console.log('Structured inserted!');
        window.location.reload();
      });
    }
    API.updateShiftStructure(SettingsData.structure).done(function() {
      console.log('Structure saved!');
    });
  }

  function saveExpansion(row) {
    let data = row.data();
    let obj = {
      morning: row.find('.morning-check')[0].checked,
      afternoon: row.find('.afternoon-check')[0].checked,
      night: row.find('.night-check')[0].checked,
      rest: row.find('.rest-check')[0].checked,
      prog: parseInt(row.find('.prog').text())
    };
    API.updateShiftExpansion(data.id, obj).done(function() {
      console.log('Expansion saved!');
      let exp = SettingsData.expansion.find(p => p.id = data.id);
      $.extend(exp, obj);
    });
  }

  function removeExpansion() {
    let el = $(shiftTable.body.find('tr:last')[0]);
    API.deleteShiftExpansion(el.data().id).done(function() {
      console.log('Expansion removed!');
      el.remove();
      SettingsData.expansion.pop();
    });
  }

  function addExpansion() {
    let prog;
    if(SettingsData.expansion != null)
      prog = SettingsData.expansion.length + 1;
    else
      prog = 1;
    let exp = {
      morning: false,
      afternoon: false,
      night: false,
      rest: false,
      prog: prog
    };
    API.insertShiftExpansion(SettingsData.structure.id, exp).done(function() {
      console.log('Expansion added!');
      API.loadShiftExpansion(SettingsData.structure.id).done(function(expansion) {
        SettingsData.expansion = expansion;
        initTable();
      });
    });
  }

  // public
  return {
    init: init
  };

})();
