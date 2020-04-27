package com.jojonomic.katalon.web

import org.openqa.selenium.WebDriver
import org.openqa.selenium.WebElement
import org.openqa.selenium.interactions.Actions
import org.openqa.selenium.support.ui.Select
import org.openqa.selenium.support.ui.WebDriverWait

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webui.common.WebUiCommonHelper
import com.kms.katalon.core.webui.driver.DriverFactory

public class Common {

	public static WebDriver driver
	public static WebDriverWait wait
	public static Actions actions

	/**
	 * Base variable of Helper
	 */
	@Keyword
	public static void setUp(){
		driver = DriverFactory.getWebDriver()
		wait = new WebDriverWait(driver, 10)
		actions = new Actions(driver)
	}

	/**
	 * Find web element based on test object
	 * 
	 * @param object TestObject on object repository
	 * @return Return WebElement
	 */

	public static WebElement findWebElement(TestObject to){
		return WebUiCommonHelper.findWebElement(to,10)
	}

	/**
	 * Select value in drop down by index
	 * 
	 * @param xPath XPath of web element
	 * @param value Value which to select
	 */
	public static void selectByIndex(String xPath, int value){
		Select select = new Select(Helper.defineWebElement(xPath, "xpath"))
		select.selectByIndex(value)
		select = null
	}

	/**
	 * Select value in drop down by value
	 * 
	 * @param xPath XPath of web element
	 * @param value Value which to select
	 */
	public static void selectByValue(String xPath, String value){
		Select select = new Select(Helper.defineWebElement(xPath, "xpath"))
		select.selectByValue(value)
		select = null
	}

	/**
	 * Detection table in browser based on list header table name
	 * 
	 * @param headerTable List header table name
	 * @return Return WebElement of table
	 */
	public static WebElement detectionTable(List<String> headerTable) {
		WebElement checkHeader
		WebElement tableHeader

		for(int i=1; i <= headerTable.size(); i++) {
			checkHeader = Helper.defineWebElement("//thead//th[text()='"+headerTable.get(i-1).toString()+"']", "xpath")
			if(checkHeader != null) {
				break;
			}
		}

		try {
			tableHeader = Helper.defineWebElement(checkHeader, "./ancestor::div[4]//div[contains(@class,'scrollBody')]", "xpath")
		} catch(error) {
			tableHeader = Helper.defineWebElement(checkHeader, "./ancestor::table", "xpath")
		}

		return tableHeader
	}

	public static List<WebElement> getHeaderTable(List<String> headerTable) {
		WebElement tableHeader = detectionTable(headerTable)

		def objects = Helper.defineWebElements(tableHeader,'.//th[contains(@aria-label, "ascending")]','xpath')

		println("Total headers can sort : $objects.size")

		return objects
	}
}