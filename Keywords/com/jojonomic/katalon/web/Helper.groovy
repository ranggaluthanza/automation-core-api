package com.jojonomic.katalon.web

import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import org.openqa.selenium.By
import org.openqa.selenium.JavascriptExecutor
import org.openqa.selenium.WebElement
import org.openqa.selenium.support.ui.ExpectedConditions

import com.jojonomic.katalon.etc.VariableConstants
import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.util.KeywordUtil
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import kms.turing.katalon.plugins.helper.table.HTMLTableHelper as TableHelper

public class Helper extends Common implements VariableConstants{

	/** 
	 * Get resolution browser
	 * 
	 */
	@Keyword
	public static void getResolution() {
		def size = driver.manage().window().getSize().toString()
		println("Your resolution browser is : ".concat(size))
	}

	/**
	 * Open and navigate to new tab
	 * 
	 * @param urlWeb URL web site
	 */
	@Keyword
	public static void openNewTabAndNavigate(String urlWeb) {
		WebUI.executeJavaScript("window.open();", null)
		WebUI.switchToWindowUrl('about:blank')
		WebUI.navigateToUrl(urlWeb)
	}

	/**
	 * Wait web element on browser and click it
	 * 
	 * @param xPath String XPath Web Element
	 * @throws IOException throws error
	 */
	@Keyword
	public static void waitElementAndClick(String xPath) throws IOException {
		WebElement object = wait.until(ExpectedConditions.elementToBeClickable(By.xpath(xPath)))
		object.click()
	}

	/**
	 * Wait until web element not visible
	 * 
	 * @param to TestObject on Object Repository
	 */
	@Keyword
	public static void waitUntilElementNotVisible(TestObject to) {
		def element = findWebElement(to)
		wait.until(ExpectedConditions.invisibilityOf(element))
	}

	/**
	 * Add Global Variable on fly/running script
	 * 
	 * @param name Variable name
	 * @param value Value in variable
	 */
	@Keyword
	public static void addGlobalVariable(String name, def value) {
		GroovyShell shell1 = new GroovyShell()
		MetaClass mc = shell1.evaluate("internal.GlobalVariable").metaClass
		String getterName = "get" + name.capitalize()
		mc.'static'."$getterName" = { -> return value }
		mc.'static'."$name" = value
	}

	/**
	 * Upload file/documents
	 * 
	 * @param to Tag <input> for upload on object repository
	 * @param file Filename with extension. Put into  folder ../Data Files/
	 */
	@Keyword
	public static void uploadFile(TestObject to, String file) {
		if(System.getProperty('os.name').toString().contains('Windows')) {
			path = path.replace('/', '\\');
		}
		WebUI.uploadFile(to, path+file);
	}


	/**
	 * Sorting data tables
	 * 
	 * @param headerTable List of header table name
	 */
	@Keyword
	public static void sortDataWithHeaders(List<String> headerTable) {
		List<WebElement> objects = getHeaderTable(headerTable)
		objects.each { object ->
			println("The header is being clicked : $object.text")
			object.click()
			waitUntilElementNotVisible(findTestObject('Object Repository/Main Page/loadingPage'))
		}
	}

	/**
	 * Sorting data tables and validation data every single column
	 * 
	 * @param headerTable List of header table name
	 */
	@Keyword
	public static void sortDataWithHeadersAndValidation(List<String> headerTable) {
		List<WebElement> objects = getHeaderTable(headerTable)
		objects.each { object ->
			println("The header is being clicked : $object.text")
			object.click()
			waitUntilElementNotVisible(findTestObject('Object Repository/Main Page/loadingPage'))

			List<WebElement> Objects = defineWebElements('//table//tbody//tr//td', 'xpath')

			println("Total objects = " + Objects.size() + " Object index 0 = " + Objects.get(0).getText())

			if(Objects.size() == 1 && Objects.get(0).getText() == 'No data found in the server'){
				KeywordUtil.markErrorAndStop("Data not found! please crosscheck your sort!")
			}
		}
	}

	/**
	 * Get data from data table (first data on table)
	 * 
	 * @param tableHeader List of header table name
	 * @return Return value cell
	 */
	@Keyword
	public static String getDataFromDataTable(List<String> tableHeader) {
		WebElement table = TableHelper.identifyTableByColumnHeaders(tableHeader,
				10, FailureHandling.CONTINUE_ON_FAILURE)

		String cellVal = TableHelper.getCellsValueByColumnHeader(table, tableHeader.get(0).toString(),
				FailureHandling.CONTINUE_ON_FAILURE).get(0).toString()

		return cellVal
	}

	/**
	 * Click data on table and direct to detail
	 * 
	 * @param tableHeader List of header table name
	 */
	@Keyword
	public static void clickOneDataOnDataTable(List<String> tableHeader) {
		WebElement table = TableHelper.identifyTableByColumnHeaders(tableHeader,
				10, FailureHandling.CONTINUE_ON_FAILURE)

		List<String> cellVal = TableHelper.getCellsValueByColumnHeader(table, tableHeader.get(0).toString(),
				FailureHandling.CONTINUE_ON_FAILURE)

		WebElement cell = TableHelper.identifyCellByHeaderAndCellsInfo(table, tableHeader.get(0).toString(),
				[(tableHeader.get(0).toString()) : cellVal.get(0).toString()], FailureHandling.CONTINUE_ON_FAILURE)

		cell.click()
	}

	/**
	 * Click data on table, direct to detail and returns a value when clicked
	 * 
	 * @param tableHeader List of header table name
	 * @return Return value clicked
	 */
	@Keyword
	public static String clickOneDataOnDataTableReturnValue(List<String> tableHeader) {
		WebElement table = TableHelper.identifyTableByColumnHeaders(tableHeader,
				10, FailureHandling.CONTINUE_ON_FAILURE)

		String cellVal = TableHelper.getCellsValueByColumnHeader(table, tableHeader.get(0).toString(),
				FailureHandling.CONTINUE_ON_FAILURE).get(0).toString()

		WebElement cell = TableHelper.identifyCellByHeaderAndCellsInfo(table, tableHeader.get(0).toString(),
				[(tableHeader.get(0).toString()) : cellVal], FailureHandling.CONTINUE_ON_FAILURE)

		cell.click()

		return cellVal
	}


	/**
	 * Click data on table and choose which column to click 
	 * 
	 * @param tableHeader List of header table name
	 * @param toHeader The order of the columns you want to click
	 */
	@Keyword
	public static void clickOneDataOnDataTableWithIndex(List<String> tableHeader, Integer toHeader){
		WebElement table = TableHelper.identifyTableByColumnHeaders(tableHeader,
				10, FailureHandling.CONTINUE_ON_FAILURE)

		List<String> cellVal = TableHelper.getCellsValueByColumnHeader(table, tableHeader.get(toHeader).toString(),
				FailureHandling.CONTINUE_ON_FAILURE)

		WebElement cell = TableHelper.identifyCellByHeaderAndCellsInfo(table, tableHeader.get(toHeader).toString(),
				[(tableHeader.get(toHeader).toString()) : cellVal.get(0).toString()], FailureHandling.CONTINUE_ON_FAILURE)

		cell.click()
	}

	/**
	 * Handle single-select <select2> library 
	 * 
	 * @param to TestObject which contains select2
	 * @param selectContains Text contains in drop down
	 * @throws IOException Return message error
	 */
	@Keyword
	public static void select2Click(TestObject to, String selectContains) throws IOException {
		def select2 = findWebElement(to)
		def subContainerClass = "./following-sibling::*[contains(@class,'container')]";
		WebElement Testclick = defineWebElement(select2,subContainerClass,'xpath')
		Testclick.click()

		WebElement typingDropdown = defineWebElement("//*[contains(@class,'container--open')]//input", "xpath")
		typingDropdown.sendKeys(selectContains)
		waitElementAndClick("//li[contains(@class,'--highlighted')]")
	}

	/**
	 * Handle typing on <select2> library
	 * 
	 * @param to TestObject which contains select2
	 * @param selectContains Text contains in drop down
	 * @throws IOException Return message error
	 */
	@Keyword
	public static void select2Typing(TestObject object, String selectContains) throws IOException {
		def select2 = findWebElement(object)
		def subContainerClass = "./following-sibling::*[contains(@class,'container')]";
		WebElement Testclick = defineWebElement(select2,subContainerClass,'xpath')
		Testclick.click()

		WebElement typingDropdown = defineWebElement("//*[contains(@class,'container--open')]//input", "xpath")
		typingDropdown.sendKeys(selectContains)
	}

	/**
	 * Handle multiple-select on <select2> library
	 * 
	 * @param to TestObject which contains select2
	 * @param selectContains Text contains in drop down
	 * @throws IOException Return message error
	 */
	@Keyword
	public static void select2Multiple(TestObject object, List<String> selectContains) throws IOException {
		def select2 = findWebElement(object)
		def subContainerClass = "./following-sibling::*[contains(@class,'container')]";
		WebElement Testclick = defineWebElement(select2,subContainerClass,'xpath')

		selectContains.each { values ->
			Testclick.click()
			WebElement input = defineWebElement(select2,'./following-sibling::span//input','xpath')
			input.sendKeys(values)
			WebElement objectContains = defineWebElement("//li[contains(@class,'--highlighted')]", "xpath")
			objectContains.click()
		}
	}

	/**
	 * Choose date in calendar
	 * 
	 * @param to Calendar object
	 * @param year Year of date
	 * @param month Month of date
	 * @param day Day of date
	 * @throws IOException Return error
	 */
	@Keyword
	public static void calendarPick(TestObject to,String year, int month, String day) throws IOException {
		WebElement calendar = findWebElement(to)

		calendar.click()
		WebUI.delay(1)

		WebElement monthCal = defineWebElement("//div[@class='pika-single is-bound']//select[@class='pika-select pika-select-month']", 'xpath')
		def monthConv = WebUI.convertWebElementToTestObject(monthCal)

		WebUI.selectOptionByIndex(monthConv, month-1)
		WebUI.delay(1)

		calendar.click()
		WebUI.delay(1)

		WebElement yearCal = defineWebElement("//div[@class='pika-single is-bound']//select[@class='pika-select pika-select-year']", 'xpath')
		def yearConv = WebUI.convertWebElementToTestObject(yearCal)

		WebUI.selectOptionByValue(yearConv, year, false)

		WebUI.delay(1)
		calendar.click()

		waitElementAndClick("//div[@class='pika-single is-bound']//td[@data-day=$day]")
	}

	/**
	 * Get string array in data table
	 * 
	 * @param headerName Header data table
	 * @return return string array of data table
	 */
	@Keyword
	public static String[] getDataFromTable(String headerName) {
		def data = []
		int indexData

		List<WebElement> element = defineWebElements("//table//thead//th","xpath")
		List<WebElement> dataElement = defineWebElements("//table//tbody/tr","xpath")

		println("Total header table : ${element.size}")
		println("Total data on table : ${dataElement.size}")

		searchHeader : for(def i : (0..element.size())){
			if(element.get(i).text.toUpperCase() == headerName.toUpperCase()){
				KeywordUtil.markPassed("Found your header table : ${headerName} on index : $i")
				indexData = i+1
				break searchHeader
			}
		}

		for(int j = 0; j < dataElement.size(); j++){
			String dataFromTable = defineWebElement("//table//tbody/tr["+(j+1)+"]//td[$indexData]", 'xpath').text
			data.push(dataFromTable);
		}
		return data;
	}

	/**
	 * Drag and drop object
	 * 
	 * @param objNumber List string of object. Just name
	 * @param toObject Place to drop it
	 */
	@Keyword
	public static void dragAndDropNumber(List<String> objNumber, TestObject toObject) {
		objNumber.each { obj ->
			def drag = defineWebElement("//span[text()='$obj']//parent::div", "xpath")
			dragAndDrop(drag, findWebElement(toObject))
		}
	}

	/**
	 * Click data in data table based on text reference
	 * 
	 * @param headerTable List name of  header data table
	 * @param textClick Text reference to click
	 */
	@Keyword
	public static void clickInTableBasedOnText(List<String> headerTable, String textClick) {
		WebElement tableHeader = detectionTable(headerTable)

		try {
			defineWebElement(tableHeader, ".//tbody//td[text()='$textClick']", 'xpath').click()
		} catch(express) {
			defineWebElement(tableHeader, ".//tbody//td//div[text()='$textClick']", 'xpath').click()
		}
	}

	/**
	 * Checklist data(s) on data table
	 * 
	 * @param headerTable List name of header data table 
	 * @param totalSelected Total data that you want to select
	 */
	@Keyword
	public static void checklistDataInTable(List<String> headerTable, int totalSelected) {
		WebElement tableHeader = detectionTable(headerTable)

		def checkData = defineWebElements(tableHeader, ".//descendant::label", 'xpath')

		println(checkData.size())

		for(int i = 1; i <= totalSelected; i++ ) {
			checkData.get(i-1).click()
			((JavascriptExecutor) driver).executeScript("arguments[0].scrollIntoView(true);", checkData.get(i-1));
		}
	}

	/**
	 * Scroll to element
	 * 
	 * @param object Web element object
	 */
	@Keyword
	public static void scrollToElement(WebElement object){
		actions.moveToElement(object).perform()
	}

	/**
	 * Drag and drop object
	 * 
	 * @param from Object to drag (Web eLement)
	 * @param to Place to drop object (Web element)
	 */
	@Keyword
	public static void dragAndDrop(WebElement from, WebElement to){
		actions.dragAndDrop(from, to).build().perform()
	}

	/**
	 * Drag and drop object
	 * 
	 * @param from Object to drag (Test Object)
	 * @param to Place to drop object (Web element)
	 */
	@Keyword
	public static void dragAndDrop(TestObject from, WebElement to){
		actions.dragAndDrop(from, to).build().perform()
	}

	/**
	 * Drag and drop object
	 * 
	 * @param from Object to drag (Web element)
	 * @param to Place to drop object (Test object)
	 */
	@Keyword
	public static void dragAndDrop(WebElement from, TestObject to){
		actions.dragAndDrop(from, to).build().perform()
	}

	/**
	 * Drag and drop object
	 * 
	 * @param from Object to drag (Test object)
	 * @param to Place to drop object (Test object)
	 */
	@Keyword
	public static void dragAndDrop(TestObject from, TestObject to){
		actions.dragAndDrop(from, to).build().perform()
	}

	/**
	 * Define web element
	 * 
	 * @param locator Locator web element
	 * @param type Web element search method
	 * @return Return WebElement
	 */
	@Keyword
	public static WebElement defineWebElement(String locator, String type){
		WebElement defineElement

		type = type.toLowerCase()
		switch(type){
			case type = "xpath" :
				defineElement = driver.findElement(By.xpath(locator))
				break;
			case type = "id" :
				defineElement = driver.findElement(By.id(locator))
				break;
			case type = "cssselector" :
				defineElement = driver.findElement(By.cssSelector(locator))
				break;
			case type = "cssname" :
				defineElement = driver.findElement(By.className(locator))
				break;
			case type = "linktext" :
				defineElement = driver.findElement(By.linkText(locator))
				break;
			case type = "name" :
				defineElement = driver.findElement(By.name(locator))
				break;
			case type = "partiallink" :
				defineElement = driver.findElement(By.partialLinkText(locator))
				break;
			case type = "tag" :
				defineElement = driver.findElement(By.tagName(locator))
				break;
		}
		return defineElement
	}

	/**
	 * Define web element inner object
	 * 
	 * @param object Base object to search
	 * @param locator Locator web element
	 * @param type Web element search method
	 * @return Return WebElement
	 */
	@Keyword
	public static WebElement defineWebElement(WebElement object, String locator, String type){
		WebElement defineElement

		type = type.toLowerCase()
		switch(type){
			case type = "xpath" :
				defineElement = object.findElement(By.xpath(locator))
				break;
			case type = "id" :
				defineElement = object.findElement(By.id(locator))
				break;
			case type = "cssselector" :
				defineElement = object.findElement(By.cssSelector(locator))
				break;
			case type = "cssname" :
				defineElement = object.findElement(By.className(locator))
				break;
			case type = "linktext" :
				defineElement = object.findElement(By.linkText(locator))
				break;
			case type = "name" :
				defineElement = object.findElement(By.name(locator))
				break;
			case type = "partiallink" :
				defineElement = object.findElement(By.partialLinkText(locator))
				break;
			case type = "tag" :
				defineElement = object.findElement(By.tagName(locator))
				break;
		}
		return defineElement
	}

	/**
	 * Define list of web element
	 * 
	 * @param locator Locator web element
	 * @param type Web element search method
	 * @return Return List<WebElement>
	 */
	@Keyword
	public static List<WebElement> defineWebElements(String locator, String type){
		List<WebElement> defineElement

		type = type.toLowerCase()
		switch(type){
			case type = "xpath" :
				defineElement = driver.findElements(By.xpath(locator))
				break;
			case type = "id" :
				defineElement = driver.findElements(By.id(locator))
				break;
			case type = "cssselector" :
				defineElement = driver.findElements(By.cssSelector(locator))
				break;
			case type = "cssname" :
				defineElement = driver.findElements(By.className(locator))
				break;
			case type = "linktext" :
				defineElement = driver.findElements(By.linkText(locator))
				break;
			case type = "name" :
				defineElement = driver.findElements(By.name(locator))
				break;
			case type = "partiallink" :
				defineElement = driver.findElements(By.partialLinkText(locator))
				break;
			case type = "tag" :
				defineElement = driver.findElements(By.tagName(locator))
				break;
		}
		return defineElement
	}

	/**
	 * Define list of web element
	 * 
	 * @param object Base object to search
	 * @param locator Locator web element
	 * @param type Web element search method
	 * @return Return List<WebElement>
	 */
	@Keyword
	public static List<WebElement> defineWebElements(WebElement object, String locator, String type){
		List<WebElement> defineElement

		type = type.toLowerCase()
		switch(type){
			case type = "xpath" :
				defineElement = object.findElements(By.xpath(locator))
				break;
			case type = "id" :
				defineElement = object.findElements(By.id(locator))
				break;
			case type = "cssselector" :
				defineElement = object.findElements(By.cssSelector(locator))
				break;
			case type = "cssname" :
				defineElement = object.findElements(By.className(locator))
				break;
			case type = "linktext" :
				defineElement = object.findElements(By.linkText(locator))
				break;
			case type = "name" :
				defineElement = object.findElements(By.name(locator))
				break;
			case type = "partiallink" :
				defineElement = object.findElements(By.partialLinkText(locator))
				break;
			case type = "tag" :
				defineElement = object.findElements(By.tagName(locator))
				break;
			default :
				println("Please insert the type correctly!")
		}
		return defineElement
	}
}