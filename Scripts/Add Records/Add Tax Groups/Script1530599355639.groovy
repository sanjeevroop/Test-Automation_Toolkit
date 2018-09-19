import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('http://mauw2k8sdworx:81/')

WebUI.setText(findTestObject('Temp/Page_ (12)/input_Username'), 'dev@sdworx.com')

WebUI.setText(findTestObject('Temp/Page_ (12)/input_Password'), '1234')

WebUI.click(findTestObject('Temp/Page_ (12)/input_Submit'))

WebUI.maximizeWindow()

WebUI.click(findTestObject('Temp/h4_YOUR SETUP (13)'))

WebUI.click(findTestObject('Temp/button_Your Setup (9)'))

WebUI.click(findTestObject('Temp/a_Your Payroll (7)'))

WebUI.click(findTestObject('Temp/a_Tax Groups'))

for (def row = 1; row <= findTestData('TaxGroup').getRowNumbers(); row++) {
    WebUI.click(findTestObject('Temp/a_Add Tax Group (1)'))

    WebUI.setText(findTestObject('Temp/input_TaxGroupNo0'), findTestData('TaxGroup').getValue(1, row))

    WebUI.setText(findTestObject('Temp/input_TaxDistrictName0'), findTestData('TaxGroup').getValue(2, row))

    WebUI.selectOptionByIndex(findTestObject('Temp/select_No  Yes'), findTestData('TaxGroup').getValue(3, row))

    WebUI.setText(findTestObject('Temp/input_Address10'), findTestData('TaxGroup').getValue(4, row))

    WebUI.setText(findTestObject('Temp/input_Address20'), findTestData('TaxGroup').getValue(5, row))

    WebUI.setText(findTestObject('Temp/input_Address30'), findTestData('TaxGroup').getValue(6, row))

    WebUI.setText(findTestObject('Temp/input_TownCity0'), findTestData('TaxGroup').getValue(7, row))

    WebUI.setText(findTestObject('Temp/input_PostCode0'), findTestData('TaxGroup').getValue(8, row))

    WebUI.setText(findTestObject('Temp/input_HMRCCompanyTaxName0'), findTestData('TaxGroup').getValue(9, row))

    WebUI.setText(findTestObject('Temp/input_ContactName0'), findTestData('TaxGroup').getValue(10, row))

    WebUI.setText(findTestObject('Temp/input_CompanyEmailAddress0'), findTestData('TaxGroup').getValue(11, row))

    WebUI.setText(findTestObject('Temp/input_HMRCNumber0'), findTestData('TaxGroup').getValue(12, row))

    WebUI.setText(findTestObject('Temp/input_Reference0'), findTestData('TaxGroup').getValue(13, row))

    WebUI.setText(findTestObject('Temp/input_AccountOfficeReference0'), findTestData('TaxGroup').getValue(14, row))

    WebUI.setText(findTestObject('Temp/input_UTRCompanyTaxName0'), findTestData('TaxGroup').getValue(15, row))

    WebUI.setText(findTestObject('Temp/input_CorporationReference0'), findTestData('TaxGroup').getValue(16, row))

    WebUI.setText(findTestObject('Temp/input_SelfAssessmentReference0'), findTestData('TaxGroup').getValue(17, row))

    WebUI.click(findTestObject('Temp/button_Save (7)'))
}

