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
const CalendarAPI = (function() {

  let calendar;
  let data;

  /**
  * Init Calendar API.
  */
  function init(structure, expansion) {
    data = {
      's': structure,
      'e': expansion
    };
    calendar = new HelloWeek({
      selector: '#calendar',
      lang: 'it',
      langFolder: './static/langs/',
      format: 'dd/mm/yyyy',
      weekShort: false,
      monthShort: false,
      multiplePick: false,
      defaultDate: false,
      todayHighlight: true,
      daysSelected: null, // ['2019-02-26', '2019-03-01', '2019-03-02', '2019-03-03']
      disablePastDays: false,
      disabledDaysOfWeek: false,
      disableDates: false,
      weekStart: 1, // 0 (Sunday) to 6 (Saturday).
      daysHighlight: false,
      range: false,
      rtl: false,
      locked: false,
      minDate: false,
      maxDate: false,
      nav: ['◀', '▶'],
      onLoad: (c) => {
        console.log('onLoad');
        initTurnoStruttura();
        initEvent();
        customizeButton();
      },
      onChange: () => {
        console.log('onChange');
      },
      onSelect: () => {
        console.log('onSelect');
      },
      onClear: () => {
        console.log('onClear');
      }
    });
  }

  /**
  * Call this for initial setup.
  */
  function initTurnoStruttura() {
    let start = moment().startOf('month');
    setMonthView(start);
  }

  /**
  * Call this for set month view.
  */
  function initMonthView() {
    let start = moment().year(calendar.getYear()).month(calendar.getMonth()-1).date(1);
    start.set({hour:0,minute:0,second:0,millisecond:0});
    setMonthView(start);
  }

  function setMonthView(start) {
    let days = $('.month .day');
    let firstStruttura = moment(data.s.day);
    let d, diff;
    for(el of days) {
      el = $(el);
      diff = start.diff(firstStruttura, 'd');
      start.add(1, 'd');
      if(diff >= 0) {
        d = data.e[diff % data.e.length];
      } else {
        d = data.e[(data.e.length + (diff % data.e.length)) % data.e.length];
      }
      if(d.morning) {
        el.addClass('morning');
      } else if(d.afternoon) {
        el.addClass('afternoon');
      } else if(d.night) {
        el.addClass('night');
      } else if(d.rest) {
        el.addClass('rest');
      } else {
        console.warn('Struttura not found!');
      }
    }
  }

  /**
  * Init calendar event.
  */
  function initEvent() {
    $('.prev').click(function(e) {
      initMonthView();
    });
    $('.next').click(function(e) {
      initMonthView();
    });
  }

  /**
  * Customize calendar button.
  */
  function customizeButton() {
    $('.prev').html('<i class="fas fa-chevron-left nav-icon"></i>');
    $('.next').html('<i class="fas fa-chevron-right nav-icon"></i>');
  }

  /**
  * For develop and debug purpose only.
  */
  function getCalendar() {
    console.warn("Don't use this!");
    return calendar;
  }

  /**
  * Navigate to a date and reload month view.
  */
  function goToDate(date) {
    calendar.goToDate(date);
    initMonthView();
  }

  // public
  return {
    init: init,
    getCalendar: getCalendar,
    goToDate: goToDate
  };

})();
