import org.openqa.selenium.Keys as Keys
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import internal.GlobalVariable as GlobalVariable

WebUI.openBrowser('')

WebUI.navigateToUrl('http://mauw2k8sdworx:81/')

WebUI.setText(findTestObject('Temp/input_Username (28)'), 'dev@sdworx.com')

WebUI.setText(findTestObject('Temp/input_Password (29)'), '1234')

WebUI.click(findTestObject('Temp/input_Submit (29)'))

WebUI.click(findTestObject('Temp/h4_YOUR SETUP (21)'))

WebUI.click(findTestObject('Temp/button_Your Setup (18)'))

WebUI.click(findTestObject('Temp/a_Your Pay  Benefits (11)'))

WebUI.click(findTestObject('Temp/a_Company Pay Rates (3)'))

for (def row = 1; row <= findTestData('TaxGroup').getRowNumbers(); row++) {

WebUI.click(findTestObject('Temp/a_Add Company Pay Rates (3)'))

WebUI.selectOptionByIndex(findTestObject('Temp/select_Please SelectAccounts E (2)'), findTestData('CompanyPayRates').getValue(1, row))

WebUI.setText(findTestObject('Temp/input_date (6)'), findTestData('CompanyPayRates').getValue(2, row))

WebUI.setText(findTestObject('Temp/input_AgeFrom0 (1)'), findTestData('CompanyPayRates').getValue(3, row))

WebUI.setText(findTestObject('Temp/input_AgeTo0 (1)'), findTestData('CompanyPayRates').getValue(4, row))

WebUI.setText(findTestObject('Temp/input_HourlyRate0 (1)'), findTestData('CompanyPayRates').getValue(5, row))

WebUI.click(findTestObject('Temp/button_Save (12)'))

}

//WebUI.closeBrowser()

